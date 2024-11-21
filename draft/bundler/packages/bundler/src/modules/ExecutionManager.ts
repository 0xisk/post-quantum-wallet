import Debug from "debug";
import { Mutex } from "async-mutex";
import { ValidateUserOpResult, ValidationManager } from "@account-abstraction/validation-manager";
import { UserOperation } from "@account-abstraction/utils";
import { clearInterval } from "timers";

import { BundleManager, SendBundleReturn } from "./BundleManager";
import { MempoolManager } from "./MempoolManager";
import { ReputationManager } from "./ReputationManager";
import { BigNumber } from "ethers";

const debug = Debug("aa.exec");

/**
 * execute userOps manually or using background timer.
 * This is the top-level interface to send UserOperation
 */
export class ExecutionManager {
  private reputationCron: any;
  private autoBundleInterval: any;
  private maxMempoolSize = 0; // default to auto-mining
  private autoInterval = 0;
  private readonly mutex = new Mutex();

  constructor(
    private readonly reputationManager: ReputationManager,
    private readonly mempoolManager: MempoolManager,
    private readonly bundleManager: BundleManager,
    private readonly validationManager: ValidationManager
  ) {}

  
  
  /**
   * send a user operation through the bundler.
   * @param userOp the UserOp to send.
   */
  async sendUserOperation(userOp: UserOperation, entryPointInput: string): Promise<void> {
    await this.mutex.runExclusive(async () => {
      console.log("____________________________________________________________ 1");
      debug("sendUserOperation");
  
      // Increase gas limits
      userOp.verificationGasLimit = "0x7ffffff"; // Higher verification gas
      userOp.callGasLimit = "0xfffff"; // Higher call gas
      userOp.preVerificationGas = "0x100000"; // Higher pre-verification gas
      userOp.maxFeePerGas = "0x8f0d1800"; // Adjust as needed
      userOp.maxPriorityFeePerGas = "0x8f0d1800"; // Adjust as needed
  
      console.log("userOp=", userOp);
      console.log("____________________________________________________________ 2");
  
      this.validationManager.validateInputParameters(userOp, entryPointInput);
      console.log("____________________________________________________________ 3");
  
      const validationResult: ValidateUserOpResult = {
        returnInfo: {
          preOpGas: BigNumber.from(0),
          prefund: BigNumber.from(0),
          sigFailed: false,
          validAfter: 0,
          validUntil: 0
        },
        senderInfo: {
          addr: '',
          stake: '0',
          unstakeDelaySec: 0
        },
        referencedContracts: {
          addresses: [],
          hash: ''
        },
        storageMap: {}
      };
  
      console.log("validationResult=", validationResult);
      console.log("____________________________________________________________ 4");
  
      const userOpHash = await this.validationManager.entryPoint.getUserOpHash(userOp);
      console.log("____________________________________________________________ 5");
  
      this.mempoolManager.addUserOp(
        true,
        userOp,
        userOpHash,
        validationResult.returnInfo.prefund,
        validationResult.referencedContracts,
        validationResult.senderInfo,
        validationResult.paymasterInfo,
        validationResult.factoryInfo,
        validationResult.aggregatorInfo
      );
  
      console.log("____________________________________________________________ 6");
      await this.attemptBundle(true);
      console.log("____________________________________________________________ 7");
    });
  }
  

  setReputationCron(interval: number): void {
    debug("set reputation interval to", interval);
    clearInterval(this.reputationCron);
    if (interval !== 0) {
      this.reputationCron = setInterval(
        () => this.reputationManager.hourlyCron(),
        interval
      );
    }
  }

  /**
   * set automatic bundle creation
   * @param autoBundleInterval autoBundleInterval to check. send bundle anyway after this time is elapsed. zero for manual mode
   * @param maxMempoolSize maximum # of pending mempool entities. send immediately when there are that many entities in the mempool.
   *    set to zero (or 1) to automatically send each UserOp.
   * (note: there is a chance that the sent bundle will contain less than this number, in case only some mempool entities can be sent.
   *  e.g. throttled paymaster)
   */
  setAutoBundler(autoBundleInterval: number, maxMempoolSize: number): void {
    debug(
      "set auto-bundle autoBundleInterval=",
      autoBundleInterval,
      "maxMempoolSize=",
      maxMempoolSize
    );
    clearInterval(this.autoBundleInterval);
    this.autoInterval = autoBundleInterval;
    if (autoBundleInterval !== 0) {
      this.autoBundleInterval = setInterval(() => {
        void this.attemptBundle(true).catch((e) =>
          console.error("auto-bundle failed", e)
        );
      }, autoBundleInterval * 1000);
    }
    this.maxMempoolSize = maxMempoolSize;
  }

  /**
   * attempt to send a bundle now.
   * @param force
   */
  async attemptBundle(force = true): Promise<SendBundleReturn | undefined> {
    debug(
      "attemptBundle force=",
      force,
      "count=",
      this.mempoolManager.count(),
      "max=",
      this.maxMempoolSize
    );
    if (force || this.mempoolManager.count() >= this.maxMempoolSize) {
      const ret = await this.bundleManager.sendNextBundle();
      if (this.maxMempoolSize === 0) {
        // in "auto-bundling" mode (which implies auto-mining) also flush mempool from included UserOps
        await this.bundleManager.handlePastEvents();
      }
      return ret;
    }
  }
}

/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */

// export class HelloNEAR {
//   constructor({ contractId, walletToUse }) {
//     this.contractId = contractId;
//     this.wallet = walletToUse;
//   }

//   async getGreeting() {
//     return await this.wallet.viewMethod({ contractId: this.contractId, method: 'get_greeting' });
//   }

//   async setGreeting(greeting) {
//     return await this.wallet.callMethod({ contractId: this.contractId, method: 'set_greeting', args: { message: greeting } });
//   }
// }

export class HelloNEAR {
  constructor({ contractId, walletToUse }) {
    this.contractId = contractId;
    this.wallet = walletToUse;
  }

  async getGreeting() {
    let res1 = await this.wallet.callMethod({ contractId: this.contractId, method: 'query_greeting' });
    return await this.wallet.getTransactionResult(res1.transaction_outcome.id);
  }

  async setGreeting(greeting) {
    return await this.wallet.callMethod({ contractId: this.contractId, method: 'change_greeting', args: { new_greeting: greeting } });
  }
}
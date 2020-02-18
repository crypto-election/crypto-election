import * as Exonum from 'exonum-client'
import axios from 'axios'
import * as proto from '../../proto/stubs.js'

const TRANSACTION_URL = '/api/explorer/v1/transactions'
const PER_PAGE = 10
const SERVICE_ID = 128

const TX_CREATE_PARTICIPANT_ID = 0;
const TX_CREATE_ADMINISTRATION_ID = 1;
const TX_ISSUE_ELECTION_ID = 2;
const TX_VOTE_ID = 3;
const TX_SUBMIT_LOCATION_ID = 4;

const TABLE_INDEX = 0

// MyCode
let Participant = Exonum.newType(proto.crypto_election.core.Participant);
//const Wallet = Exonum.newType(proto.exonum.examples.cryptocurrency_advanced.Wallet)

//MyCode

const CreateParticipantTransaction = new Exonum.Transaction({
  serviceId: SERVICE_ID,
  methodId: TX_CREATE_PARTICIPANT_ID,
  schema: proto.crypto_election.core.CreateParticipant,
});

const CreateAdministrationTransaction = new Exonum.Transaction({
  serviceId: SERVICE_ID,
  methodId: TX_CREATE_ADMINISTRATION_ID,
  schema: proto.crypto_election.core.CreateAdministration,
});

const IssueElectionTransaction = new Exonum.Transaction({
  serviceId: SERVICE_ID,
  methodId: TX_ISSUE_ELECTION_ID,
  schema: proto.crypto_election.core.IssueElection,
});

const VoteTransaction = new Exonum.Transaction({
  serviceId: SERVICE_ID,
  methodId: TX_VOTE_ID,
  schema: proto.crypto_election.core.Vote,
})

const SubmitLocationTransaction = new Exonum.Transaction({
  serviceId: SERVICE_ID,
  methodId: TX_SUBMIT_LOCATION_ID,
  schema: proto.crypto_election.core.SubmitLocation,
})
//End My code

//MyCode 2
function getTransaction(transaction, publicKey) {
  if (transaction.participant) {
    return new CreateParticipantTransaction(publicKey)
  }

  if (transaction.administration) {
    return new CreateAdministrationTransaction(publicKey)
  }

  return new IssueElectionTransaction(publicKey)
}


function deserializeTx (transaction) {
  const txTypes = [
    CreateParticipantTransaction,
    CreateAdministrationTransaction, 
    IssueElectionTransaction,
    VoteTransaction,
    SubmitLocationTransaction,
  ];
  for (const tx of txTypes) {
    const txData = tx.deserialize(Exonum.hexadecimalToUint8Array(transaction))
    if (txData) {
      return Object.assign({}, txData.payload, {
        hash: txData.hash(),
        to: txData.payload.to ? Exonum.uint8ArrayToHexadecimal(txData.payload.to.data) : undefined
      })
    }
  }
  return { name: 'initialTx' }
}

/**
 * Data for {@link CreateParticipantTransaction}
 * @typedef {Object} CreateParticipantTransactionData
 * @prop {string} name            - Participant`s name
 * @prop {string} email           - Participant`s email
 * @prop {string} phone_number    - Participant`s phone number
 * @prop {ArrayBuffer?} residence - Participant`s residence
 * @prop {string} pass_code       - Participant`s passport code
 */

module.exports = {
  install(Vue) {
    Vue.prototype.$blockchain = {
      generateKeyPair() {
        return Exonum.keyPair()
      },

      generateSeed() {
        return Exonum.randomUint64()
      },

      /**
       * Sends {@link CreateParticipantTransaction}.
       * @param {*} keyPair 
       * @param {CreateParticipantTransactionData} data - Transcation data
       */
      createParticipant(keyPair, data) {
        // Describe transaction
        const transaction = CreateParticipantTransaction.create(data, keyPair.publicKey).serialize()

        // Send transaction into blockchain
        return Exonum.send(TRANSACTION_URL, transaction)
      },

      addFunds(keyPair, amountToAdd, seed) {
        // Describe transaction
        const transaction = new IssueTransaction(keyPair.publicKey)

        // Transaction data
        const data = {
          amount: amountToAdd.toString(),
          seed: seed
        }

        // Send transaction into blockchain
        return transaction.send(TRANSACTION_URL, data, keyPair.secretKey)
      },

      transfer(keyPair, receiver, amountToTransfer, seed) {
        // Describe transaction
        const transaction = new TransferTransaction(keyPair.publicKey)

        // Transaction data
        const data = {
          to: { data: Exonum.hexadecimalToUint8Array(receiver) },
          amount: amountToTransfer,
          seed: seed
        }

        // Send transaction into blockchain
        return transaction.send(TRANSACTION_URL, data, keyPair.secretKey)
      },

      getWallet(publicKey) {
        return axios.get('/api/services/supervisor/consensus-config').then(response => {
          // actual list of public keys of validators
          const validators = response.data.config.validator_keys.map(validator => validator.consensus_key)

          return axios.get(`/api/services/cryptocurrency/v1/wallets/info?pub_key=${publicKey}`)
            .then(response => response.data)
            .then(data => {
              return Exonum.verifyBlock(data.block_proof, validators).then(() => {
                // verify table timestamps in the root tree
                const tableRootHash = Exonum.verifyTable(data.wallet_proof.to_table, data.block_proof.block.state_hash, "election.participants")

                // find wallet in the tree of all wallets
                const walletProof = new Exonum.MapProof(data.wallet_proof.to_wallet, Exonum.PublicKey, Wallet)
                if (walletProof.merkleRoot !== tableRootHash) {
                  throw new Error('Wallet proof is corrupted')
                }
                const wallet = walletProof.entries.get(publicKey)
                if (typeof wallet === 'undefined') {
                  throw new Error('Wallet not found')
                }

                // get transactions
                const transactionsMetaData = Exonum.merkleProof(
                  Exonum.uint8ArrayToHexadecimal(new Uint8Array(wallet.history_hash.data)),
                  wallet.history_len,
                  data.wallet_history.proof,
                  [0, wallet.history_len],
                  Exonum.Hash
                )

                if (data.wallet_history.transactions.length !== transactionsMetaData.length) {
                  // number of transactions in wallet history is not equal
                  // to number of transactions in array with transactions meta data
                  throw new Error('Transactions can not be verified')
                }

                // validate each transaction
                const transactions = []
                let index = 0

                for (let transaction of data.wallet_history.transactions) {
                  const hash = transactionsMetaData[index++]
                  const buffer = Exonum.hexadecimalToUint8Array(transaction.message)
                  const bufferWithoutSignature = buffer.subarray(0, buffer.length - 64)
                  const author = Exonum.uint8ArrayToHexadecimal(buffer.subarray(0, 32))
                  const signature = Exonum.uint8ArrayToHexadecimal(buffer.subarray(buffer.length - 64, buffer.length));

                  const Transaction = getTransaction(transaction.debug, author)

                  if (Exonum.hash(buffer) !== hash) {
                    throw new Error('Invalid transaction hash')
                  }

                  // serialize transaction and compare with message
                  if (!Transaction.serialize(transaction.debug).every(function (el, i) {
                    return el === bufferWithoutSignature[i]
                  })) {
                    throw new Error('Invalid transaction message')
                  }

                  if (!Transaction.verifySignature(signature, author, transaction.debug)) {
                    throw new Error('Invalid transaction signature')
                  }

                  const transactionData = Object.assign({ hash: hash }, transaction.debug)
                  if (transactionData.to) {
                    transactionData.to = Exonum.uint8ArrayToHexadecimal(new Uint8Array(transactionData.to.data))
                  }
                  transactions.push(transactionData)
                }

                return {
                  block: data.block_proof.block,
                  wallet: wallet,
                  transactions: transactions
                }
              })
            })
        })
      },

      getBlocks(latest) {
        const suffix = !isNaN(latest) ? '&latest=' + latest : ''
        return axios.get(`/api/explorer/v1/blocks?count=${PER_PAGE}${suffix}`).then(response => response.data)
      },

      getBlock(height) {
        return axios.get(`/api/explorer/v1/block?height=${height}`).then(response => response.data)
      },

      getTransaction(hash) {
        return axios.get(`/api/explorer/v1/transactions?hash=${hash}`).then(response => response.data)
      }
    }
  }
}

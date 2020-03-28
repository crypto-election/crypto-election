import * as Exonum from 'exonum-client'
import axios from 'axios'
import * as proto from '../../proto/stubs.js'

const TRANSACTION_URL = '/api/explorer/v1/transactions'
const PER_PAGE = 10
const SERVICE_ID = 4
const SERVICE_NAME = 'crypto_election'
const SERVICE_PUBLIC_API_PATH = `/api/services/${SERVICE_NAME}/v1`

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
        const transaction = CreateParticipantTransaction.create(data, keyPair).serialize()

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

      getParticipant(publicKey) {
        return axios.get('/api/services/supervisor/consensus-config').then(response => {
          // actual list of public keys of validators
          const validators = response.data.validator_keys.map(validator => validator.consensus_key)

          return axios.get(`${SERVICE_PUBLIC_API_PATH}/participants/info?key=${publicKey}`)
              .then(response => response.data)
              .then(({ block_proof, object_proof, history }) => {
                Exonum.verifyBlock(block_proof, validators)
                // verify table timestamps in the root tree
                const tableRootHash = Exonum.verifyTable(
                    object_proof.to_table,
                    block_proof.block.state_hash,
                    `${SERVICE_NAME}.participants`)

                // find wallet in the tree of all wallets
                const participantProof = new Exonum.MapProof(
                    object_proof.to_object,
                    Exonum.MapProof.rawKey(Exonum.PublicKey),
                    Participant)
                if (participantProof.merkleRoot !== tableRootHash)
                  throw new Error('Participant proof is corrupted')

                const participant = participantProof.entries.get(Exonum.publicKeyToAddress(publicKey))
                if (typeof participant == 'undefined') throw new Error('Participant not found')

                const verifiedTransactions = new Exonum.ListProof(history.proof, Exonum.Hash)
                const hexHistoryHash = Exonum.uint8ArrayToHexadecimal(new Uint8Array(participant.history_hash.data))
                if (verifiedTransactions.merkleRoot !== hexHistoryHash) throw new Error('Transactions proof is corrupted')

                const validIndexes = verifiedTransactions
                    .entries
                    .every(({ index }, i) => i === index)
                if (!validIndexes) throw new Error('Invalid transaction indexes in the proof')

                // deserialize transactions
                const transactions = history.transactions.map(deserializeTx)

                const correctHashes = transactions.every(({ hash }, i) => verifiedTransactions.entries[i].value === hash)
                if (!correctHashes) throw new Error('Transaction hash mismatch')

                return {
                  block: block_proof.block,
                  wallet: participant,
                  transactions: transactions
                }
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
        return axios.get(`/api/explorer/v1/transactions?hash=${hash}`)
            .then(response => response.data)
            .then(data => {
              data.content = deserializeTx(data.message)
              return data
            })
      }
    }
  }
}

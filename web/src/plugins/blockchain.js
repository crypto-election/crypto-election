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
const Participant = Exonum.newType(proto.crypto_election.core.Participant);
const Administration = Exonum.newType(proto.crypto_election.core.Administration);
const PublicKey = Exonum.MapProof.rawKey(Exonum.PublicKey)
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

async function getConsensusConfig() { return await axios.get('/api/services/supervisor/consensus-config') }

async function getValidatorsConsensusKeys() {
  return (await getConsensusConfig()).data.validator_keys
    .map(validator => validator.consensus_key)
}

/**
 * @template K
 * Queries from validator proofed info for some entity
 * @param {string} entityName 
 * @param {K} key 
 * @returns {ProofedInfo<K, *>}
 */
async function getEntityProofedInfo(entityName, key) {
  const path = `${SERVICE_PUBLIC_API_PATH}/${entityName}/info?key=${key}`;
  return (await axios.get(path)).data
}

function getTableRootHash(object_proof, block_proof, tableName) {
  // verify table timestamps in the root tree
  return Exonum.verifyTable(
      object_proof.to_table,
      block_proof.block.state_hash,
      `${SERVICE_NAME}.${tableName}`
    )
}

function requireVerifiedEntity(object_proof, tableRootHash, entityName, keyType, valueType, key) {
  // find wallet in the tree of all wallets
  const objectProof = new Exonum.MapProof(object_proof.to_object, keyType, valueType)
  if (objectProof.merkleRoot !== tableRootHash) throw new Error(entityName + ' proof is corrupted')            
  const entity = objectProof.entries.get(key)
  if (typeof entity == 'undefined') throw new Error(entityName + ' not found')
  return entity
}

function verifyTransactionProof(verifiedTransactions, historyHash) {
  const hexHistoryHash = Exonum.uint8ArrayToHexadecimal(new Uint8Array(historyHash.data))
  if (verifiedTransactions.merkleRoot !== hexHistoryHash)
    throw new Error('Transactions proof is corrupted')
}

function verifyTransactionIndexes(verifiedTransactions) {
  const isIndexesValid = verifiedTransactions.entries
    .every(({ index }, i) => i === index)
  if (!isIndexesValid) throw new Error('Invalid transaction indexes in the proof')
}

function verifyTransactionHashes(transactions, verifiedTransactions) {
  const isHashesCorrect = transactions
    .every(({ hash }, i) => verifiedTransactions.entries[i].value === hash)
  if (!isHashesCorrect) throw new Error('Transaction hash mismatch')
}

function getVerifiedTransactions(history, historyHash) {
  const verifiedTransactions = new Exonum.ListProof(history.proof, Exonum.Hash)
  verifyTransactionProof(verifiedTransactions, historyHash)
  verifyTransactionIndexes(verifiedTransactions)    
  // deserialize transactions
  const transactions = history.transactions.map(deserializeTx)    
  verifyTransactionHashes(transactions, verifiedTransactions)    
  return transactions;
}

/**
 * Data for {@link CreateAdministrationTransaction}
 * @typedef {Object} CreateAdministrationTransactionData
 * @prop {string} name                - Administration`s name
 * @prop {ArrayBuffer?} principal_key - Administration`s principal administration key
 * @prop {Polygon} area               - Administration`s phone number
 */

 /**
  * @template K Key type
  * @template V Value type
  * @typedef {Object} ProofedInfo
  * @prop {*} block_proof
  * @prop {Proof<K, V>} object_proof
  * @prop {History?} history
  */

 /**
  * @template K Key type
  * @template V Value type
  * @typedef {Object} Proof
  * @prop {MapProof<string, Hash>} to_table
  * @prop {MapProof<K, V>} to_object
  * @prop {History?} history
  */

  /**
  * @template K Key type
  * @template V Value type
  * @typedef {Object} MapProof
  * @prop {OptionalEntry<K, V>[]} entries
  * @prop {MapProofEntry[]} proof
  */

 /**
   * @typedef {Object} History
   * @prop {*} proof
   * @prop {*[]} transactions
   */

/**
 * Area polygon
 * ## Example
 * ```javascript
 * let polygon = {
 *   exterior: [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
 *   interiors: [
 *     [[0.1, 0.1], [0.4, 0.1], [0.4, 0.1], [0.1, 0.4]],
 *     [[0.6, 0.6], [0.9, 0.6], [0.9, 0.9], [0.6, 0.9]],
 *   ],
 * }
 * ```
 * @typedef {Object} Polygon
 * @prop {number[][]} exterior       - External boundary 
 * @prop {number[][][]} interiors    - Internal boundaries
 */

/**
 * Data for {@link CreateParticipantTransaction}
 * @typedef {Object} CreateParticipantTransactionData
 * @prop {string} name                - Participant`s name
 * @prop {string} email               - Participant`s email
 * @prop {string} phone_number        - Participant`s phone number
 * @prop {ArrayBuffer?} residence     - Participant`s residence administration key
 * @prop {string} pass_code           - Participant`s passport code
 */


/**
 * Data for {@link IssueElectionTransaction}
 * @typedef {Object} IssueElectionTransactionData
 * @prop {string} name                - Election`s name
 * @prop {date} start_date            - Election`s start_date
 * @prop {date} finish_date           - Election`s finish_date
 * @prop {Array} options              - Election`s options
 */

 /**
  * Pair of secret and public keys
  * @typedef {Object} KeyPair
  * @prop {string} publicKey          - Public key hexadecimal representation
  * @prop {string} secretKey          - Secret key hexadecimal representation
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
       * @param {KeyPair} keyPair 
       * @param {CreateParticipantTransactionData} data - Transcation data
       */
      createParticipant(keyPair, data) {
        // Describe transaction
        const transaction = CreateParticipantTransaction.create(data, keyPair).serialize()

        // Send transaction into blockchain
        return Exonum.send(TRANSACTION_URL, transaction)
      },

      /**
       * Sends {@link CreateAdministrationTransaction}.
       * @param {KeyPair} keyPair 
       * @param {CreateAdministrationTransactionData} data - Transcation data
       */
      createAdministration(keyPair, data) {
        // Describe transaction
        const transaction = CreateAdministrationTransaction.create(data, keyPair).serialize()

        // Send transaction into blockchain
        return Exonum.send(TRANSACTION_URL, transaction)
      },

       /**
       * Sends {@link IssueElectionTransaction}.
       * @param {KeyPair} keyPair 
       * @param {IssueElectionTransactionData} data - Transcation data
       */
      createNewPoll(keyPair, data) {
        // Describe transaction
        const transaction = IssueElectionTransaction.create(data, keyPair).serialize()

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
      //my big code
      async getAdministration(publicKey) {
        const validators = await getValidatorsConsensusKeys()
        const { block_proof, object_proof, history } =
          await getEntityProofedInfo('administrations', publicKey)

        Exonum.verifyBlock(block_proof, validators)
        // verify table timestamps in the root tree
        const tableRootHash = getTableRootHash(object_proof, block_proof, 'administrations')
        
        const administration = requireVerifiedEntity(
          object_proof,
          tableRootHash,
          'Administration',
          PublicKey,
          Administration,
          Exonum.publicKeyToAddress(publicKey)
        )

        return {
          block: block_proof.block,
          administration,
          transactions: getVerifiedTransactions(history, administration.history_hash)
        }
      },
      //end my big code
      async getParticipant(publicKey) {
        // actual list of public keys of validators
        const validators = await getValidatorsConsensusKeys()
        const { block_proof, object_proof, history } =
          await getEntityProofedInfo('participants', publicKey)
        
        Exonum.verifyBlock(block_proof, validators)

        const tableRootHash = getTableRootHash(object_proof, block_proof, 'participants')

        const participant = requireVerifiedEntity(
          object_proof,
          tableRootHash,
          'Participant',
          PublicKey,
          Participant,
          Exonum.publicKeyToAddress(publicKey)
        )

        return {
          block: block_proof.block,
          participant,
          transactions: getVerifiedTransactions(history, participant.history_hash)
        }
      },

      async getBlocks(latest) {
        const suffix = !isNaN(latest) ? '&latest=' + latest : ''
        return (await axios.get(`/api/explorer/v1/blocks?count=${PER_PAGE}${suffix}`)).data
      },

      async getBlock(height) {
        return (await axios.get(`/api/explorer/v1/block?height=${height}`)).data
      },

      async getTransaction(hash) {
        const { data } = await axios.get(`/api/explorer/v1/transactions?hash=${hash}`)
        data.content = deserializeTx(data.message)
        return data
      }
    }
  }
}

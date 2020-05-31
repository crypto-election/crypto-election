import * as Exonum from 'exonum-client'
import axios from 'axios'
import * as proto from '../../proto/stubs.js'

//#region Definitions
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
 * @typedef {Object} IssueElectionFormData
 * @prop {string} name                - Election`s name
 * @prop {number} startDate           - Election`s start date
 * @prop {number} finishDate         - Election`s finish date
 * @prop {Array} options              - Election`s options
 */

/**
 * Data for {@link IssueElectionTransaction}
 * @typedef {Object} IssueElectionTransactionData
 * @prop {any} addr                   - Election`s address
 * @prop {string} name                - Election`s name
 * @prop {any} start_date             - Election`s start_date
 * @prop {any} finish_date            - Election`s finish_date
 * @prop {Array} options              - Election`s options
 */

 /**
  * Pair of secret and public keys
  * @typedef {Object} KeyPair
  * @prop {string} publicKey          - Public key hexadecimal representation
  * @prop {string} secretKey          - Secret key hexadecimal representation
  */
//#endregion

const TRANSACTION_URL = '/api/explorer/v1/transactions';
const PER_PAGE = 10;
const SERVICE_ID = 4;
const SERVICE_NAME = 'crypto_election';
const SERVICE_PUBLIC_API_PATH = `/api/services/${SERVICE_NAME}/v1`;

const TX_CREATE_PARTICIPANT_ID = 0;
const TX_CREATE_ADMINISTRATION_ID = 1;
const TX_ISSUE_ELECTION_ID = 2;
const TX_VOTE_ID = 3;
const TX_SUBMIT_LOCATION_ID = 4;

const Participant = Exonum.newType(proto.crypto_election.core.Participant);
const Administration = Exonum.newType(proto.crypto_election.core.Administration);
const PublicKey = Exonum.MapProof.rawKey(Exonum.PublicKey);

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

const txTypes = [
  CreateParticipantTransaction,
  CreateAdministrationTransaction, 
  IssueElectionTransaction,
  VoteTransaction,
  SubmitLocationTransaction,
];

function parseTx (transaction) {
  for (const tx of txTypes) {
    const txData = tx.deserialize(Exonum.hexadecimalToUint8Array(transaction))
    if (txData) {
      return Object.assign({}, txData.payload, { hash: txData.hash() })
    }
  }
  return { name: 'initialTx' }
}

async function getValidatorsConsensusKeys() {
  const consensusConfigPath = '/api/services/supervisor/consensus-config';
  const validatorKeys = (await axios.get(consensusConfigPath)).data.validator_keys;
  return validatorKeys.map(({ consensus_key }) => consensus_key);
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
  return (await axios.get(path)).data;
}

function getTableRootHash(object_proof, block_proof, tableName) {
  // verify table timestamps in the root tree
  return Exonum.verifyTable(
      object_proof.to_table,
      block_proof.block.state_hash,
      `${SERVICE_NAME}.${tableName}`
    )
}

function requireVerifiedEntity(
  object_proof,
  tableRootHash,
  entityName,
  keyType,
  valueType,
  key
) {
  // find wallet in the tree of all wallets
  const objectProof = new Exonum.MapProof(object_proof.to_object, keyType, valueType);
  if (objectProof.merkleRoot !== tableRootHash)
    throw new Error(entityName + ' proof is corrupted');           

  const entity = objectProof.entries.get(key);
  if (typeof entity == 'undefined')
    throw new Error(entityName + ' not found');

  return entity;
}

function verifyTxProof(verifiedTxs, historyHash) {
  const rawHistoryHash = new Uint8Array(historyHash.data);
  const hexHistoryHash = Exonum.uint8ArrayToHexadecimal(rawHistoryHash);
  if (verifiedTxs.merkleRoot !== hexHistoryHash)
    throw new Error('Transactions proof is corrupted');
}

function verifyTxIndexes(verifiedTransactions) {
  const isIndexesValid = verifiedTransactions.entries
    .every(({ index }, i) => i === index)
  if (!isIndexesValid) throw new Error('Invalid transaction indexes in the proof');
}

function verifyTxHashes(transactions, verifiedTransactions) {
  const isHashesCorrect = transactions
    .every(({ hash }, i) => verifiedTransactions.entries[i].value === hash)
  if (!isHashesCorrect) throw new Error('Transaction hash mismatch')
}

function getVerifiedTransactions(history, historyHash) {
  const verifiedTxs = new Exonum.ListProof(history.proof, Exonum.Hash);
  verifyTxProof(verifiedTxs, historyHash);
  verifyTxIndexes(verifiedTxs);
  
  // deserialize transactions
  const transactions = history.transactions.map(parseTx);
  verifyTxHashes(transactions, verifiedTxs);
  return transactions;
}

function randomAddress() {
  const keyPair = Exonum.keyPair();
  const rawByteArray = Exonum.hexadecimalToUint8Array(keyPair.publicKey + keyPair.secretKey);
  const rawHash = Exonum.hexadecimalToUint8Array(Exonum.hash(rawByteArray));

  return { data: rawHash };
}

/**
 * Prepares transaction for transferring
 * @param {IssueElectionFormData} data Data from form
 * @returns {IssueElectionTransactionData} Ready transaction data
 */
function prepareIssueElectionTransaction(data) {
  const address = randomAddress();

  return {
    addr: address,
    name: data.name,
    start_date: msToPbTimestamp(data.startDate),
    finish_date: msToPbTimestamp(data.finishDate),
    options: data.options,
  }
}

function msToPbTimestamp(timeMS) {
  const timestamp = new proto.google.protobuf.Timestamp();
  timestamp.seconds = timeMS / 1000;
  timestamp.nanos = (timeMS % 1000) * 1e6;
  return timestamp;
}

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
       * @param {CreateParticipantTransactionData} data - Transaction data
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
       * @param {CreateAdministrationTransactionData} data - Transaction data
       */
      createAdministration(keyPair, data) {
        // Describe transaction
        const transaction = CreateAdministrationTransaction.create(data, keyPair).serialize();

        // Send transaction into blockchain
        return Exonum.send(TRANSACTION_URL, transaction)
      },

       /**
       * Sends {@link IssueElectionTransaction}.
       * @param {KeyPair} keyPair 
       * @param {IssueElectionFormData} data - Transaction data
       */
      createNewPoll(keyPair, data) {
        const prepared = prepareIssueElectionTransaction(data);
        // Describe transaction
        const transaction = IssueElectionTransaction.create(prepared, keyPair).serialize();

        // Send transaction into blockchain
        return Exonum.send(TRANSACTION_URL, transaction)
      },

      vote(keyPair, hexAddress, option) {
        const payload = {
          election_id: { data: Exonum.hexadecimalToUint8Array(hexAddress) },
          option_id: option,
          seed: this.generateSeed(),
        }

        const transaction = VoteTransaction.create(payload, keyPair).serialize();

        return Exonum.send(TRANSACTION_URL, transaction)
      },
      
      async getSuggestedElections(publicKey) {
        const path = `${SERVICE_PUBLIC_API_PATH}/elections/suggested-for?key=${publicKey}`;
        return (await axios.get(path)).data;
      },

      async getElectionResults(electionAddress) {
        const path = `${SERVICE_PUBLIC_API_PATH}/elections/result?key=${electionAddress}`;
        return (await axios.get(path)).data;
      },

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
        data.content = parseTx(data.message)
        return data
      }
    }
  }
}

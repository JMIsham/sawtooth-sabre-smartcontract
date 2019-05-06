# Summary 
This RFC proposes Smart Permission Functions (SMF) to enforce private transactions within the consortium network. Using PIKE SDK, SMF can be developed in Rust and compiled to WASM byte codes that will be executed alongside with other Sabre smart contracts.
SPFs are non other that business logic that encapsulates logic for enforcing permissions.

Overall view on SMF: https://sawtooth.hyperledger.org/docs/sabre/nightly/master/smart_permissions.html

# How do SMFs work ?
Similar to Sabre Contracts, Smart Permission Functions can be developed and converted byte codes (.wasm) files can be saved in the 'state' - on chain. 

Boilerplate code for SMF : https://github.com/hyperledger/sawtooth-sabre/tree/master/contracts/sawtooth-pike/examples

# HL Execution of SMF
![Alt text](https://github.com/JMIsham/sawtooth-sabre-smartcontract/blob/master/SMF%20Execution.png)

## PIKE Transaction Processor
PIKE transaction processor defines how organizations and agents should be managed in the context of enabling permissions.
Agents are the actors or partners of the network that play a different role in supply chain process.
Agents have an organization which enables having multiple agents per organization. 

# SMF Implementation specification
In the context of food supply chain, from master data to transactional data there are requirements to enforce permissions for partners to be competitive. 

# Potenatial Data Model
![Alt text](https://github.com/JMIsham/sawtooth-sabre-smartcontract/blob/master/Rzr%20Bck%20-%20Data%20Model.png)

## Required Technical Capability
* Enforcing updates/deletes only to the 'owner' of the master-data
* Private transactions only readable to a set of partners
    * Sharing product catalogues only between a set of partners
    * Transactional data should only be accessed by relevant parties


# Open Questions 

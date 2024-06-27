# rindexer

checklist v1.0:
- add examples in the repo + callouts in the documentation
- do benchmarks with a few different projects

bugs:
- get the pooling everytime from the network rpc
- Do not create a new postgres client each time on rust projects
- fix last TODOs in the code
- graphql is blocking indexer starting up so making indexing slower
- fix the environment variables for RPC urls 

nice to have:
- look at the final unwraps
- look at the final clones
- add ability to add contracts to the yaml pulling in the ABI, deploy block, and contract address
- look into PK with tx hash and tx index and log index to make it unique and not have to worry about duplicates
- add ability to add indexes to the database
- ability for one-to-many relationships
  -     relationships:
        - contract_name: BasePaint
          event: Transfer
          event_input_name: from
          linked_to:
            - contract_name: BasePaint
              event: Approval
              event_input_name: owner
            - contract_name: BasePaint
              event: Approval
              event_input_name: owner
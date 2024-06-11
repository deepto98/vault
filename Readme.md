# Vault Smart Contract
 We build a vault, which an user can deposit sol to or withdraw from.  
Vault will have 4 instructions :  
1. Initialize - User will create new vault 
2. Deposit - User will deposit native SOL to vault 
3. Withdraw - Send SOL from Vault to user 
4. Close - Empty the Vault to the user and close state account

 
Extra (Challenge -> Hint: Explore “Clock::get()?.slot”)

Add a locktime to withdraw so that user can only claim after it has expired.

Run anchor test to build, deploy to localnet and run tests located in the test folder.


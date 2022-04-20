use crate::domain::model::{
  account::{AccountModel, NewAccount},
  bank::{BankModel, NewBank},
  pix_key::{NewPix, PixKeyModel, PixKeyRepositoryInterface},
};
use crate::infrastructure::db::schema::{account, bank, pixkey};
use diesel::prelude::*;
use std::error::Error;
pub struct PixkeyRepositoryDb {}

impl PixKeyRepositoryInterface for PixkeyRepositoryDb {
  fn register_key(
    conn: &PgConnection,
    key: String,
    kind: String,
    account_id: String,
  ) -> QueryResult<PixKeyModel> {
    //register Pixkey
    let new_pix = NewPix::new(key, kind, account_id);
    let pixkey: PixKeyModel = diesel::insert_into(pixkey::table)
      .values(&new_pix)
      .get_result(conn)
      .expect("Error register pixkey");
    Ok(pixkey)
  }

  fn find_key_by_kind(conn: &PgConnection, kind: String, key: String) -> QueryResult<PixKeyModel> {
    let pix = pixkey::table
      .filter(pixkey::kind.eq(kind))
      .first(conn)
      .expect("Error find pixkey");
    Ok(pix)
  }

  fn add_bank(conn: &PgConnection, bank: NewBank) -> Result<(), Box<dyn Error>> {
    let bank = diesel::insert_into(bank::table)
      .values(&bank)
      .execute(conn)?;
    print!("{:?}", bank);
    Ok(())
  }

  fn add_account(conn: &PgConnection, account: NewAccount) -> Result<(), Box<dyn Error>> {
    //find bank
    let acc_id = &account.bank_id;
    let bank_find: BankModel = bank::table.filter(bank::id.eq(acc_id)).first(conn)?;
    //insert account and update bank
    match Some(bank_find) {
      Some(bank) => {
        //insert account
        let account: AccountModel = diesel::insert_into(account::table)
          .values(&account)
          .get_result(conn)?;
        print!("{:?}", account);
        //update bank
        let mut vec_accounts = bank.accounts.clone().unwrap_or_default();
        vec_accounts.push(account.id);
        // first form to update
        // let update_bank = BankModel {
        //   accounts: Some(vec_accounts),
        //   ..bank
        // };
        // let r: BankModel = diesel::update(bank::table)
        //   .filter(bank::id.eq(acc_id))
        //   .set(update_bank)
        //   .get_result(conn)?;
        //print!("{:?}", r);
        //trendy form to update
        // let target = bank::table.filter(bank::id.eq(acc_id));
        // diesel::update(target).set(bank::accounts.eq(vec_accounts));
        //Four form to update for reference
        let result: BankModel = diesel::update(&bank)
          .set(bank::accounts.eq(vec_accounts))
          .get_result(conn)?;
        print!("{:?}", result);
      }
      None => (),
    };
    Ok(())
  }

  fn find_account(conn: &PgConnection, id: &String) -> QueryResult<AccountModel> {
    let account: AccountModel = account::table
      .filter(account::id.ilike(id))
      .first(conn)
      .expect("Error find account");
    Ok(account)
  }
}

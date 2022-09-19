pub mod action { // module has to be public to access from outside

use postgres::{Client, NoTls, Error};

  pub fn create(dbsc: &String, newuser: &str, newrole: &str, newpass: &str) -> Result<(), Error> { // function has to be public to access from outside

    let mut client = Client::connect(&dbsc, NoTls)?;
    println!("## {} {} {} with password: [{}]", dbsc, newuser, newrole, newpass);
    let row = client.query_opt("SELECT 1 as foo FROM pg_roles WHERE rolname=$1", &[&newuser])?;
    match row {
        Some(row) => {
            let foo: i32 = row.get("foo");
            println!("User {} already exists: {}", newuser, foo);
        }
        None => {println!("no matching foo");

            println!("## Creating new role {} with password: [{}]", newuser, newpass);
            let btch = format!("CREATE ROLE {} LOGIN PASSWORD '{}'", &newuser, &newpass);
            client.batch_execute(&btch)?;
        }
    }

    let btch = format!("GRANT {} TO {}", &newrole, &newuser);
    client.batch_execute(&btch)?;

    Ok(())
  }

  pub fn reviewuser(dbsc: &String, newuser: &str) -> Result<(), Error> {

      let mut client = Client::connect(&dbsc, NoTls)?;

      for row in client.query("WITH rawr as (select nspname as schema_name
              , r.rolname as role_name
              , (pg_catalog.has_schema_privilege(r.rolname, nspname, 'CREATE'))::TEXT as create_grant
              , (pg_catalog.has_schema_privilege(r.rolname, nspname, 'USAGE'))::TEXT as usage_grant
          from pg_namespace pn,
          pg_catalog.pg_user u
          left join pg_catalog.pg_roles r on (usename=rolname)
          where r.rolname = $1 and nspname not in ('information_schema','pg_catalog','public','pg_toast','pg_temp_1','pg_toast_temp_1')
        )
        select * from rawr where create_grant = 'true' or usage_grant = 'true'
        order by 1,2 ", &[&newuser])? {
          let schemaname: &str = row.get(0);
          let name: &str  = row.get(1);
          let cgrant: &str  = row.get(2);
          let ugrant: &str  = row.get(3);

          println!("{} Create: {}, Usage; {} on Schema:{} ", name, cgrant, ugrant, schemaname);
      }
      println!("Searching for roles");

      for row in client.query("SELECT rolname FROM pg_roles WHERE
        pg_has_role( $1, oid, 'member');", &[&newuser])? {
          let rname: &str = row.get(0);
          println!("{} has role:{} ", newuser, rname);
      }
      Ok(())
  }
  pub fn drophim(dbsc: &String, newuser: &str) -> Result<(), Error> { // function has to be public to access from outside

      let mut client = Client::connect(&dbsc, NoTls)?;

      let row = client.query_opt("SELECT 1 as foo FROM pg_roles WHERE rolname=$1", &[&newuser])?;
      match row {
          Some(_row) => {
              println!("## Revoking from write role");
              let btchr = format!("REVOKE write FROM {}", &newuser);
              client.batch_execute(&btchr)?;

              println!("## Revoking from read_only role");
              let btchro = format!("REVOKE read_only FROM {}", &newuser);
              client.batch_execute(&btchro)?;

/*
              println!("## Reassigning owned");
              let btchrow = format!("REASSIGN OWNED BY {} TO {}", &newuser, "chuss_rw");
              client.batch_execute(&btchrow)?;
*/
              println!("## Remove permissions on Schema");
              for row in client.query("select nspname as schema_name
                  from pg_namespace pn,
                  pg_catalog.pg_user u
                  left join pg_catalog.pg_roles r on (usename=rolname)
                  where r.rolname = $1 and nspname not in ('information_schema','pg_catalog','pg_toast','pg_temp_1','pg_toast_temp_1');", &[&newuser])? {
                  let rname: &str = row.get(0);

                  let btchrow = format!("REVOKE ALL PRIVILEGES ON SCHEMA {} FROM {}  ", rname, newuser);
                  println!("REVOKE ALL PRIVILEGES ON SCHEMA {} FROM {}  ", rname, newuser);
                  client.batch_execute(&btchrow)?;
              }
              //let btcerassign = format!("REASSIGN OWNED BY {} TO CURRENT_USER", &newuser);
              //client.batch_execute(&btcerassign)?;
              let btch = format!("DROP USER IF exists {}", &newuser);
              client.batch_execute(&btch)?;
              println!("## User {} dropped!", newuser);
          }
          None => {
              println!("## Sorry buddy, this user {} doesn't exist", newuser);
          }
      }
      Ok(())
  }
  pub fn alter(dbsc: &String, newuser: &str) -> Result<(), Error> { // function has to be public to access from outside
    let newrole = "write";
    let mut client = Client::connect(&dbsc, NoTls)?;
    println!("## GRANT {} TO {}", newrole, newuser);
    let btch = format!("GRANT {} TO {}", &newrole, &newuser);
    client.batch_execute(&btch)?;

    Ok(())
  }
  pub fn change_password(dbsc: &String, newuser: &str, newpass: &str) -> Result<(), Error> { // function has to be public to access from outside
    let mut client = Client::connect(&dbsc, NoTls)?;
    println!("## CHANGE PWD {} TO {}", newuser, newpass);
    let btch = format!("ALTER ROLE {} WITH PASSWORD '{}';", &newuser, &newpass);
    println!("{}", btch);
    client.batch_execute(&btch)?;

    Ok(())
  }
  pub fn generate_random_serie() -> String {
      use rand::Rng;

      const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                              abcdefghijklmnopqrstuvwxyz\
                              0123456789&^%$#";
      const PASSWORD_LEN: usize = 15;
      let mut rng = rand::thread_rng();

      let password: String = (0..PASSWORD_LEN)
          .map(|_| {
              let idx = rng.gen_range(0..CHARSET.len());
              CHARSET[idx] as char
          })
          .collect();
      return password;
  }

  pub fn searchuser(dbsc: &String, newuser: &str) -> Result<(), Error> {
    let mut client = Client::connect(&dbsc, NoTls)?;
    for row in client.query("SELECT usename AS role_name
    FROM pg_catalog.pg_user
    WHERE usename ~ $1
    ORDER BY role_name desc;", &[&newuser])? {
        let name: &str = row.get(0);

        println!("found user: {}", name);
    }
    Ok(())
  }
}
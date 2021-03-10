use query_sqlite::{exec, exec_and_print, insert_into, Value};
use rusqlite::{params, Connection, Result, Transaction};
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    exercise_5_2_1()?;
    exercise_8_3_1()?;
    exercise_8_3_2()?;

    Ok(())
}

fn exercise_5_2_1() -> Result<()> {
    println!("\nexercise 5.2.1");

    const R: [(i32, i32); 5] = [(0, 1), (2, 3), (0, 1), (2, 4), (3, 4)];
    const S: [(i32, i32); 6] = [(0, 1), (2, 4), (2, 5), (3, 4), (0, 2), (3, 4)];

    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE R (
            A   INT,
            B   INT
        );",
        params!(),
    )?;
    conn.execute(
        "CREATE TABLE S (
            B   INT,
            C   INT
        );",
        params!(),
    )?;

    for (a, b) in &R {
        conn.execute("INSERT INTO R (A, B) VALUES (?1, ?2)", &[a, b])?;
    }
    for (b, c) in &S {
        conn.execute("INSERT INTO S (B, C) VALUES (?1, ?2)", &[b, c])?;
    }

    let exec = |stmt: &str| exec_and_print(&conn, stmt);

    exec("SELECT A + B, A * A, B * B FROM R;")?;
    exec("SELECT B + 1, C - 1 FROM S;")?;
    exec("SELECT * FROM R ORDER BY B, A;")?;
    exec("SELECT * FROM S ORDER BY B, C;")?;
    exec("SELECT DISTINCT * FROM R;")?;
    exec("SELECT DISTINCT * FROM S;")?;
    exec("SELECT A, SUM(B) FROM R GROUP BY A;")?;
    exec("SELECT B, AVG(C) FROM S GROUP BY B;")?;
    exec("SELECT A FROM R GROUP BY A")?;
    exec("SELECT A, MAX(C) FROM (R INNER JOIN S ON R.B = S.B) GROUP BY A;")?;
    exec("SELECT A, R.B, C FROM (R LEFT OUTER JOIN S ON R.B = S.B);")?;
    exec("SELECT A, S.B, C FROM (S LEFT OUTER JOIN R ON R.B = S.B);")?;
    exec(
        "SELECT A, R.B, C FROM (R LEFT OUTER JOIN S ON R.B = S.B)
UNION ALL
SELECT A, S.B, C FROM (S LEFT OUTER JOIN R ON R.B = S.B) WHERE A IS NULL;",
    )?;
    exec(
        "SELECT A, R.B, S.B, C FROM (R LEFT OUTER JOIN S ON R.B < S.B)
UNION ALL
SELECT A, R.B, S.B, C FROM (S LEFT OUTER JOIN R ON R.B < S.B) WHERE A IS NULL;",
    )?;

    Ok(())
}

fn pc_database() -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "BEGIN;
        CREATE TABLE Product (
            maker   STRING, 
            model   INT,
            type    STRING
        );
        CREATE TABLE PC (
            model   INT PRIMERY KEY,
            speed   FLOAT,
            ram     INT,
            hd      INT,
            price   INT
        );
        CREATE TABLE Laptop (
            model   INT PRIMERY KEY,
            speed   FLOAT,
            ram     INT,
            hd      INT,
            screen  FLOAT,
            price   INT
        );
        CREATE TABLE Printer (
            model   INT PRIMERY KEY,
            color   STRING,
            type    STRING,
            price   INT
        );
        COMMIT;
        ",
    )?;

    insert_into(
        &conn,
        "../query/relations/Product.csv",
        "INSERT INTO Product VALUES (?1, ?2, ?3)",
    )?;
    insert_into(
        &conn,
        "../query/relations/PC.csv",
        "INSERT INTO PC VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;
    insert_into(
        &conn,
        "../query/relations/Laptop.csv",
        "INSERT INTO Laptop VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;
    insert_into(
        &conn,
        "../query/relations/Printer.csv",
        "INSERT INTO Printer VALUES (?1, ?2, ?3, ?4)",
    )?;

    Ok(conn)
}

fn exercise_8_3_1() -> Result<(), Box<dyn Error>> {
    println!("\nexercise 8.3.1");

    let mut conn = pc_database()?;

    fn closest_price(conn: &Connection, price: i64) -> Result<(String, i64, f64)> {
        let mut stmt = conn.prepare(
            "SELECT maker, PC.model, speed, price
            FROM Product, PC
            WHERE Product.model = PC.model
            ",
        )?;

        let rows = stmt.query_map(params![], |row| {
            let maker: String = row.get(0)?;
            let model: i64 = row.get(1)?;
            let speed: f64 = row.get(2)?;
            let price: i64 = row.get(3)?;

            Ok((maker, model, speed, price))
        })?;

        let (maker, model, speed, ..) = rows
            .filter_map(|r| r.ok())
            .min_by_key(|(.., p)| (price - p).abs())
            .unwrap();

        Ok((maker, model, speed))
    }

    assert_eq!(closest_price(&conn, 1000)?.1, 1002);

    #[derive(Debug)]
    struct Laptop {
        model: i64,
        speed: f64,
        ram: i64,
        hd: i64,
        screen: f64,
        price: i64,
    }

    fn minimum_requirement(
        conn: &Connection,
        speed: f64,
        ram: i64,
        hd: i64,
        screen: f64,
    ) -> Result<Vec<(String, Laptop)>> {
        let mut stmt = conn.prepare(
            "SELECT maker, Laptop.model, speed, ram, hd, screen, price
            FROM Product, Laptop
            WHERE 
                Product.model = Laptop.model AND 
                speed >= ?1 AND 
                ram >= ?2 AND 
                hd >= ?3 AND 
                screen >= ?4
            ",
        )?;

        let rows = stmt.query_map(params![speed, ram, hd, screen], |row| {
            let maker: String = row.get("maker")?;
            let laptop = Laptop {
                model: row.get("model")?,
                speed: row.get("speed")?,
                ram: row.get("ram")?,
                hd: row.get("hd")?,
                screen: row.get("screen")?,
                price: row.get("price")?,
            };

            Ok((maker, laptop))
        })?;

        rows.into_iter().collect()
    }

    assert_eq!(
        {
            let (_, laptop) = &minimum_requirement(&conn, 2.0, 2048, 240, 20.0)?[0];
            laptop.model
        },
        2001
    );

    fn all_products(conn: &Connection, maker: &str) -> Result<Vec<Vec<Value>>> {
        let mut products = vec![];
        products.extend(exec(
            conn,
            "SELECT *
            FROM (Product NATURAL JOIN PC)
            WHERE maker = ?1
            ",
            &[maker],
        )?);
        products.extend(exec(
            conn,
            "SELECT *
            FROM (Product NATURAL JOIN Laptop)
            WHERE maker = ?1
            ",
            &[maker],
        )?);
        products.extend(exec(
            conn,
            "SELECT *
            FROM (Product NATURAL JOIN Printer)
            WHERE maker = ?1
            ",
            &[maker],
        )?);

        Ok(products)
    }

    assert_eq!(all_products(&conn, "B")?.len(), 4);

    fn budget(conn: &Connection, budget: i64, speed: i64) -> Result<Option<(i64, i64)>> {
        let mut stmt = conn.prepare(
            "SELECT PC.model, Printer.model
            FROM PC, Printer
            WHERE 
                PC.price + Printer.price <= ?1 AND 
                PC.speed >= ?2
            ORDER BY Printer.color <> 'true', PC.price + Printer.price
            ",
        )?;

        let mut rows = stmt.query_map(params![budget, speed], |row| {
            let pc_model: i64 = row.get(0)?;
            let printer_model: i64 = row.get(1)?;

            Ok((pc_model, printer_model))
        })?;

        rows.next().transpose()
    }

    assert_eq!(budget(&conn, 100000, 0)?, Some((1003, 3001)));

    #[derive(Clone, Default)]
    struct PC {
        model: i64,
        speed: f64,
        ram: i64,
        hd: i64,
        price: i64,
    }

    fn new_pc(conn: &mut Connection, maker: &str, pc: &PC) -> Result<bool> {
        let tran = conn.transaction()?;
        let row = tran.query_row(
            "SELECT *
            FROM PC
            WHERE model = ?1
            ",
            &[pc.model],
            |_| Ok(()),
        );

        if row.is_ok() {
            eprintln!("Error: existing model");
            return Ok(false);
        }

        tran.execute(
            "INSERT INTO Product
            VALUES (?1, ?2, ?3)
            ",
            params![maker, pc.model, "pc"],
        )?;

        tran.execute(
            "INSERT INTO PC
            VALUES (?1, ?2, ?3, ?4, ?5)
            ",
            params![pc.model, pc.speed, pc.ram, pc.hd, pc.price],
        )?;

        Ok(true)
    }

    assert!(!new_pc(
        &mut conn,
        "A",
        &PC {
            model: 1003,
            ..PC::default()
        }
    )?,);

    assert!(new_pc(
        &mut conn,
        "A",
        &PC {
            model: 8086,
            ..PC::default()
        }
    )?,);

    Ok(())
}

fn ship_database() -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "BEGIN;
        CREATE TABLE Classes (
            class   STRING PRIMARY KEY,
            type    STRING,
            country STRING,
            numGuns INT,
            bore    INT,
            displacement    INT
        );
        CREATE TABLE Ships (
            name    STRING PRIMARY KEY,
            class   STRING,
            launched    INT
        );
        CREATE TABLE Battles (
            name    STRING PRIMARY KEY,
            date    STRING
        );
        CREATE TABLE Outcomes (
            ship    STRING,
            battle  STRING,
            result  STRING,

            PRIMARY KEY (ship, battle)
        );
        COMMIT;
        ",
    )?;

    insert_into(
        &conn,
        "../query/relations/Classes.csv",
        "INSERT INTO Classes VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;
    insert_into(
        &conn,
        "../query/relations/Ships.csv",
        "INSERT INTO Ships VALUES (?1, ?2, ?3)",
    )?;
    insert_into(
        &conn,
        "../query/relations/Battles.csv",
        "INSERT INTO Battles VALUES (?1, ?2)",
    )?;
    insert_into(
        &conn,
        "../query/relations/Outcomes.csv",
        "INSERT INTO Outcomes VALUES (?1, ?2, ?3)",
    )?;

    Ok(conn)
}

fn max_firepower(conn: &Connection) -> Result<String> {
    conn.query_row(
        "SELECT class
            FROM Classes
            ORDER BY numGuns * bore * bore * bore DESC
            ",
        params![],
        |row| row.get(0),
    )
}

fn casualty(conn: &Connection, battle: &str) -> Result<(Option<String>, Option<String>)> {
    let sunk: Option<String> = conn
        .query_row(
            "SELECT country
            FROM Classes, Ships, Outcomes
            WHERE
                Classes.class = Ships.class AND
                Ships.name = Outcomes.ship AND
                Outcomes.battle = ?1
            GROUP BY country
            ORDER BY SUM(result = 'sunk') DESC
            ",
            &[battle],
            |row| row.get(0),
        )
        .or_else(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => Ok(None),
            _ => Err(e),
        })?;

    let damaged: Option<String> = conn
        .query_row(
            "SELECT country
            FROM Classes, Ships, Outcomes
            WHERE
                Classes.class = Ships.class AND
                Ships.name = Outcomes.ship AND
                Outcomes.battle = ?1
            GROUP BY country
            ORDER BY SUM(result = 'damaged') DESC
            ",
            &[battle],
            |row| row.get(0),
        )
        .or_else(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => Ok(None),
            _ => Err(e),
        })?;

    Ok((sunk, damaged))
}

#[derive(Default)]
struct Class {
    class: String,
    ty: String,
    country: String,
    num_guns: i64,
    bore: i64,
    displacement: i64,
}

#[derive(Default)]
struct Ship {
    name: String,
    launched: i64,
}

fn insert_ships(conn: &mut Connection, class: &Class, ships: &[Ship]) -> Result<()> {
    let tran = conn.transaction()?;
    let inserted = tran.execute(
        "INSERT INTO Classes
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ",
        params![
            &class.class,
            &class.ty,
            &class.country,
            class.num_guns,
            class.bore,
            class.displacement
        ],
    )?;
    assert_eq!(inserted, 1);

    let inserted = ships
        .iter()
        .map(|ship| {
            tran.execute(
                "INSERT INTO Ships
                VALUES (?1, ?2, ?3)
                ",
                params![&ship.name, &class.class, &ship.launched],
            )
        })
        .sum::<Result<usize>>()?;

    assert_eq!(inserted, ships.len());
    Ok(())
}

fn fix_date(conn: &mut Connection) -> Result<(), Box<dyn Error>> {
    let tran = conn.transaction()?;
    let mut stmt = tran.prepare(
        "SELECT 
                Ships.name AS ship,
                Ships.launched,
                Battles.name AS battle,
                CAST (('19' || SUBSTR(Battles.date, -2, 2)) AS INT) AS battled
            FROM Ships, Battles, Outcomes
            WHERE
                Ships.name = Outcomes.ship AND
                Battles.name = Outcomes.battle AND
                launched > battled
            ",
    )?;

    let rows = stmt.query_map(params![], |row| {
        let ship: String = row.get("ship")?;
        let launched: i64 = row.get("launched")?;
        let battle: String = row.get("battle")?;
        let battled: i64 = row.get("battled")?;

        Ok((ship, launched, battle, battled))
    })?;

    let mut errors = rows.collect::<Result<Vec<_>>>()?;

    while let Some((ship, launched, battle, battled)) = errors.pop() {
        loop {
            println!(
                "{} launched in {} after participating {} in {}",
                ship, launched, battle, battled
            );
            println!("which date should be updated? (type 'ship' or 'battle'):");
            let input = stdin_read_line()?;
            match input.as_str() {
                "ship" => {
                    fix_ship_date(&tran, &ship, battled)?;
                    break;
                }
                "battle" => {
                    fix_battle_date(&tran, &battle, launched)?;
                    errors.retain(|(_, l, b, _)| !(b != &battle && *l <= launched));
                    break;
                }
                _ => {
                    println!("type again.");
                }
            }
        }
    }

    Ok(())
}

fn stdin_read_line() -> io::Result<String> {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf)?;
    buf = buf.trim().to_string();
    Ok(buf)
}

fn fix_ship_date(tran: &Transaction, ship: &str, date: i64) -> Result<()> {
    tran.execute(
        "UPDATE Ships
        SET launched = ?1
        WHERE name = ?2
        ",
        params![date, ship],
    )?;
    Ok(())
}

fn fix_battle_date(tran: &Transaction, battle: &str, date: i64) -> Result<()> {
    let date = date.to_string();
    tran.execute(
        "UPDATE Battles
        SET date = SUBSTR(date, 0, LENGTH(date) - 2) || SUBSTR(?1, -2, 2)
        WHERE name = ?2
        ",
        params![date, battle],
    )?;
    Ok(())
}

fn exercise_8_3_2() -> Result<(), Box<dyn Error>> {
    println!("\nexercise 8.3.2");

    let mut conn = ship_database()?;

    assert_eq!(max_firepower(&conn)?, "Yamato");

    assert_eq!(
        casualty(&conn, "Surigao Strait")?,
        (Some("USA".to_owned()), Some("USA".to_owned()))
    );

    assert!(insert_ships(
        &mut conn,
        &Class {
            class: "Moltke".to_string(),
            ty: "bc".to_string(),
            country: "Germany".to_string(),
            num_guns: 10,
            bore: 11,
            displacement: 22979
        },
        &[
            Ship {
                name: "Moltke".to_string(),
                launched: 1910,
            },
            Ship {
                name: "Goeben".to_string(),
                launched: 1911,
            }
        ]
    )
    .is_ok());

    assert!(fix_date(&mut conn).is_ok());

    Ok(())
}

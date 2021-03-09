use std::error::Error;

use query_sqlite::{exec, exec_and_print, insert_into, Value};
use rusqlite::{params, Connection, Result};

fn main() -> Result<(), Box<dyn Error>> {
    exercise_5_2_1()?;
    exercise_8_3_1()?;

    Ok(())
}

fn exercise_5_2_1() -> Result<()> {
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

import { ERModel, Arrow, Relation, binary_relation, RelationKind } from "./ER";
import fs from "fs";

const OUTPUT_DIR = "output";

function main() {
  if (!fs.existsSync(OUTPUT_DIR)) {
    fs.mkdirSync(OUTPUT_DIR);
  }

  exercise_4_1_1();
  exercise_4_1_2();
}

const CUSTOMER = {
  label: "Customer",
  attrs: ["name", "address", "phone", "ssn"],
};

const ACCOUNT = {
  label: "Account",
  attrs: ["number", "type", "balance"],
};

function exercise_4_1_1() {
  let g = new ERModel("Bank");

  g.add_entity(CUSTOMER);
  g.add_entity(ACCOUNT);
  g.add_relation(binary_relation("Own", CUSTOMER, ACCOUNT));

  g.output([OUTPUT_DIR, "4_1_1.png"].join("/"));
}

function exercise_4_1_2() {
  {
    let g = new ERModel("Bank");
    g.add_entity(CUSTOMER);
    g.add_entity(ACCOUNT);
    g.add_relation(binary_relation("Own", CUSTOMER, ACCOUNT, RelationKind.OneMany));

    g.output([OUTPUT_DIR, "4_1_2_a.png"].join("/"));
  }

  {
    let g = new ERModel("Bank");
    g.add_entity(CUSTOMER);
    g.add_entity(ACCOUNT);
    g.add_relation(binary_relation("Own", CUSTOMER, ACCOUNT, RelationKind.OneOne));

    g.output([OUTPUT_DIR, "4_1_2_b.png"].join("/"));
  }

  {
    let g = new ERModel("Bank");
    let customer = {
      label: "Customer",
      attrs: ["name", "ssn"],
    };

    let phone = {
      label: "Phone",
      attrs: ["number"],
    };

    let address = {
      label: "Address",
      attrs: ["street", "city", "state"],
    };

    g.add_entity(customer);
    g.add_entity(phone);
    g.add_entity(address);
    g.add_entity(ACCOUNT);

    g.add_relation(binary_relation("Own-Account", customer, ACCOUNT));
    g.add_relation(binary_relation("Own-Phone", customer, phone));
    g.add_relation(binary_relation("Live-in", customer, address, RelationKind.ManyOne));

    g.output([OUTPUT_DIR, "4_1_2_c.png"].join("/"));
  }

  {
    let g = new ERModel("Bank");
    let customer = {
      label: "Customer",
      attrs: ["name", "ssn"],
    };

    let phone = {
      label: "Phone",
      attrs: ["number"],
    };

    let address = {
      label: "Address",
      attrs: ["street", "city", "state"],
    };

    g.add_entity(customer);
    g.add_entity(phone);
    g.add_entity(address);
    g.add_entity(ACCOUNT);

    g.add_relation(binary_relation("Own-Account", customer, ACCOUNT));
    g.add_relation(binary_relation("Have-Phone", address, phone, RelationKind.OneMany));
    g.add_relation(binary_relation("Live-in", customer, address, RelationKind.ManyOne));

    g.output([OUTPUT_DIR, "4_1_2_d.png"].join("/"));
  }
}

main();

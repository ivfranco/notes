import fs from 'fs';
import {
  Relation, ERModel, binary_relation, RelationKind, Arrow,
} from './ER';

const OUTPUT_DIR = 'output';

const CUSTOMER = {
  label: 'Customer',
  attrs: ['name', 'address', 'phone', 'ssn'],
};

const ACCOUNT = {
  label: 'Account',
  attrs: ['number', 'type', 'balance'],
};

function exercise_4_1_1() {
  const g = new ERModel('Bank');

  g.add_entity(CUSTOMER);
  g.add_entity(ACCOUNT);
  g.add_relation(binary_relation('Own', CUSTOMER, ACCOUNT));

  g.output([OUTPUT_DIR, '4_1_1.png'].join('/'));
}

function exercise_4_1_2() {
  {
    const g = new ERModel('Bank');
    g.add_entity(CUSTOMER);
    g.add_entity(ACCOUNT);
    g.add_relation(binary_relation('Own', CUSTOMER, ACCOUNT, RelationKind.OneMany));

    g.output([OUTPUT_DIR, '4_1_2_a.png'].join('/'));
  }

  {
    const g = new ERModel('Bank');
    g.add_entity(CUSTOMER);
    g.add_entity(ACCOUNT);
    g.add_relation(binary_relation('Own', CUSTOMER, ACCOUNT, RelationKind.OneOne));

    g.output([OUTPUT_DIR, '4_1_2_b.png'].join('/'));
  }

  {
    const g = new ERModel('Bank');
    const customer = {
      label: 'Customer',
      attrs: ['name', 'ssn'],
    };

    const phone = {
      label: 'Phone',
      attrs: ['number'],
    };

    const address = {
      label: 'Address',
      attrs: ['street', 'city', 'state'],
    };

    g.add_entity(customer);
    g.add_entity(phone);
    g.add_entity(address);
    g.add_entity(ACCOUNT);

    g.add_relation(binary_relation('Own-Account', customer, ACCOUNT));
    g.add_relation(binary_relation('Own-Phone', customer, phone));
    g.add_relation(binary_relation('Live-in', customer, address, RelationKind.ManyOne));

    g.output([OUTPUT_DIR, '4_1_2_c.png'].join('/'));
  }

  {
    const g = new ERModel('Bank');
    const customer = {
      label: 'Customer',
      attrs: ['name', 'ssn'],
    };

    const phone = {
      label: 'Phone',
      attrs: ['number'],
    };

    const address = {
      label: 'Address',
      attrs: ['street', 'city', 'state'],
    };

    g.add_entity(customer);
    g.add_entity(phone);
    g.add_entity(address);
    g.add_entity(ACCOUNT);

    g.add_relation(binary_relation('Own-Account', customer, ACCOUNT));
    g.add_relation(binary_relation('Have-Phone', address, phone, RelationKind.OneMany));
    g.add_relation(binary_relation('Live-in', customer, address, RelationKind.ManyOne));

    g.output([OUTPUT_DIR, '4_1_2_d.png'].join('/'));
  }
}

const TEAM = {
  label: 'Team',
  attrs: ['name'],
};

const PLAYER = {
  label: 'Player',
  attrs: ['name'],
};

const FAN = {
  label: 'Fan',
  attrs: ['name'],
};

const COLOR = {
  label: 'Color',
  attrs: ['name'],
};

function exercise_4_1_3() {

  const g = new ERModel('Sport');

  g.add_entity(TEAM);
  g.add_entity(PLAYER);
  g.add_entity(FAN);
  g.add_entity(COLOR);

  g.add_relation(binary_relation('Team-Players', TEAM, PLAYER, RelationKind.OneMany));
  g.add_relation(binary_relation('Team-Captain', TEAM, PLAYER, RelationKind.OneOne));
  g.add_relation(binary_relation('Uniform-Colors', TEAM, COLOR));
  g.add_relation(binary_relation('Fav-Team', FAN, TEAM, RelationKind.ManyOne));
  g.add_relation(binary_relation('Fav-Player', FAN, PLAYER, RelationKind.ManyOne));
  g.add_relation(binary_relation('Fav-Color', FAN, COLOR, RelationKind.ManyOne));

  g.output([OUTPUT_DIR, '4_1_3.png'].join('/'));
}

function exercise_4_1_4() {
  {
    const led_by: Relation = {
      label: "Led-by",
      arrows: [
        [PLAYER, Arrow.Many],
        [PLAYER, Arrow.Many],
        [TEAM, Arrow.Many],
      ]
    };

    const g = new ERModel('Sport');

    g.add_entity(TEAM);
    g.add_entity(PLAYER);
    g.add_entity(FAN);
    g.add_entity(COLOR);

    g.add_relation(binary_relation('Team-Players', TEAM, PLAYER, RelationKind.OneMany));
    g.add_relation(binary_relation('Team-Captain', TEAM, PLAYER, RelationKind.OneOne));
    g.add_relation(binary_relation('Uniform-Colors', TEAM, COLOR));
    g.add_relation(binary_relation('Fav-Team', FAN, TEAM, RelationKind.ManyOne));
    g.add_relation(binary_relation('Fav-Player', FAN, PLAYER, RelationKind.ManyOne));
    g.add_relation(binary_relation('Fav-Color', FAN, COLOR, RelationKind.ManyOne));
    g.add_relation(led_by);

    g.output([OUTPUT_DIR, '4_1_4_a.png'].join('/'));
  }

  {

  }
}

function main() {
  if (!fs.existsSync(OUTPUT_DIR)) {
    fs.mkdirSync(OUTPUT_DIR);
  }

  exercise_4_1_1();
  exercise_4_1_2();
  exercise_4_1_3();
  exercise_4_1_4();
}

main();

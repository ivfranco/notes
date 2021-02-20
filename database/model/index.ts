import fs from 'fs';
import {
  Relation, ERModel, binary_relation, RelationKind, Arrow, isa,
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
      label: 'Led-by',
      arrows: [
        [PLAYER, Arrow.Many, 'leader'],
        [PLAYER, Arrow.Many, 'player'],
        [TEAM, Arrow.Many],
      ]
    };

    const g = new ERModel('Sport');

    g.add_entity(TEAM);
    g.add_entity(PLAYER);
    g.add_entity(FAN);
    g.add_entity(COLOR);

    g.add_relation(binary_relation('Team-Players', TEAM, PLAYER));
    g.add_relation(binary_relation('Team-Captain', TEAM, PLAYER));
    g.add_relation(binary_relation('Uniform-Colors', TEAM, COLOR));
    g.add_relation(binary_relation('Fav-Team', FAN, TEAM, RelationKind.ManyOne));
    g.add_relation(binary_relation('Fav-Player', FAN, PLAYER, RelationKind.ManyOne));
    g.add_relation(binary_relation('Fav-Color', FAN, COLOR, RelationKind.ManyOne));
    g.add_relation(led_by);

    g.output([OUTPUT_DIR, '4_1_4_a.png'].join('/'));
  }

  {

    const captainship = {
      label: 'Captainship',
      attrs: ['team', 'start', 'end'],
    };

    const g = new ERModel('Sport');

    g.add_entity(TEAM);
    g.add_entity(PLAYER);
    g.add_entity(FAN);
    g.add_entity(COLOR);
    g.add_entity(captainship);

    g.add_relation(binary_relation('Is', captainship, PLAYER, RelationKind.ManyOne));
    g.add_relation(binary_relation('Team-Players', TEAM, PLAYER));
    g.add_relation(binary_relation('Uniform-Colors', TEAM, COLOR));
    g.add_relation(binary_relation('Fav-Team', FAN, TEAM, RelationKind.ManyOne));
    g.add_relation(binary_relation('Fav-Player', FAN, PLAYER, RelationKind.ManyOne));
    g.add_relation(binary_relation('Fav-Color', FAN, COLOR, RelationKind.ManyOne));
    g.add_relation(binary_relation('Led-By', PLAYER, captainship));
    g.add_relation(binary_relation('Team-Captain', TEAM, captainship, RelationKind.OneMany));

    g.output([OUTPUT_DIR, '4_1_4_b.png'].join('/'));
  }
}

function exercise_4_1_5() {
  const contract: Relation = {
    label: 'Contract',
    attrs: ['start', 'end'],
    arrows: [
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
  g.add_relation(contract);

  g.output([OUTPUT_DIR, '4_1_5.png'].join('/'));
}

const PEOPLE = {
  label: 'People',
  attrs: ['name'],
};

const CHILD_OF: Relation = {
  label: 'Child-of',
  arrows: [
    [PEOPLE, Arrow.Many, 'parent'],
    [PEOPLE, Arrow.Many, 'child'],
  ]
}

function exercise_4_1_6() {
  const g = new ERModel('People');

  g.add_entity(PEOPLE);

  const mother_of: Relation = {
    label: 'Mother-of',
    arrows: [
      [PEOPLE, Arrow.One, 'mother'],
      [PEOPLE, Arrow.Many, 'child'],
    ]
  }

  const father_of: Relation = {
    label: 'Father-of',
    arrows: [
      [PEOPLE, Arrow.One, 'father'],
      [PEOPLE, Arrow.Many, 'child'],
    ]
  }

  const child_of: Relation = {
    label: 'Child-of',
    arrows: [
      [PEOPLE, Arrow.Many, 'parent'],
      [PEOPLE, Arrow.Many, 'child'],
    ]
  }

  g.add_relation(mother_of);
  g.add_relation(father_of);
  g.add_relation(child_of);

  g.output([OUTPUT_DIR, '4_1_6.png'].join('/'));
}

function exercise_4_1_7() {
  const female = {
    label: 'Female',
  };

  const male = {
    label: 'Male',
  };

  const mother = {
    label: 'Mother',
  };

  const father = {
    label: 'Father',
  };

  const g = new ERModel('People');

  g.add_entity(female);
  g.add_entity(male);
  g.add_entity(father);
  g.add_entity(mother);

  g.add_isa(isa(PEOPLE, female));
  g.add_isa(isa(PEOPLE, male));
  g.add_isa(isa(female, mother));
  g.add_isa(isa(male, father));

  g.add_relation(binary_relation('Mother-of', mother, PEOPLE, RelationKind.OneMany));
  g.add_relation(binary_relation('Father-of', father, PEOPLE, RelationKind.OneMany));
  g.add_relation(CHILD_OF);

  g.output([OUTPUT_DIR, '4_1_7.png'].join('/'));
}

function exercise_4_1_8() {
  {
    const family: Relation = {
      label: 'Family',
      arrows: [
        [PEOPLE, Arrow.One, 'mother'],
        [PEOPLE, Arrow.One, 'father'],
        [PEOPLE, Arrow.Many, 'child'],
      ]
    };

    const g = new ERModel('People');

    g.add_entity(PEOPLE);
    g.add_relation(family);

    g.output([OUTPUT_DIR, '4_1_8_a.png'].join('/'));
  }

  {
    const couple = {
      label: 'Couple',
    };

    const g = new ERModel('People');

    g.add_entity(PEOPLE);
    g.add_entity(couple);

    g.add_relation(binary_relation('Female-is', couple, PEOPLE, RelationKind.ManyOne));
    g.add_relation(binary_relation('Male-is', couple, PEOPLE, RelationKind.ManyOne));
    g.add_relation(binary_relation('Child-of', PEOPLE, couple, RelationKind.ManyOne));

    g.output([OUTPUT_DIR, '4_1_8_b.png'].join('/'));
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
  exercise_4_1_5();
  exercise_4_1_6();
  exercise_4_1_7();
  exercise_4_1_8();
}

main();

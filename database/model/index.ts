import fs from 'fs';
import {
  Relation,
  ERModel,
  binary_relation,
  RelationKind,
  Arrow,
  isa,
  support_relation,
} from './ER';
import { validate } from './ODL';
import {
  Association,
  association,
  association_from_self_relation,
  mul,
  UML,
} from './UML';

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
    g.add_relation(
      binary_relation('Own', CUSTOMER, ACCOUNT, RelationKind.OneMany)
    );

    g.output([OUTPUT_DIR, '4_1_2_a.png'].join('/'));
  }

  {
    const g = new ERModel('Bank');
    g.add_entity(CUSTOMER);
    g.add_entity(ACCOUNT);
    g.add_relation(
      binary_relation('Own', CUSTOMER, ACCOUNT, RelationKind.OneOne)
    );

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
    g.add_relation(
      binary_relation('Live-in', customer, address, RelationKind.ManyOne)
    );

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
    g.add_relation(
      binary_relation('Have-Phone', address, phone, RelationKind.OneMany)
    );
    g.add_relation(
      binary_relation('Live-in', customer, address, RelationKind.ManyOne)
    );

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

  g.add_relation(
    binary_relation('Team-Players', TEAM, PLAYER, RelationKind.OneMany)
  );
  g.add_relation(
    binary_relation('Team-Captain', TEAM, PLAYER, RelationKind.OneOne)
  );
  g.add_relation(binary_relation('Uniform-Colors', TEAM, COLOR));
  g.add_relation(binary_relation('Fav-Team', FAN, TEAM, RelationKind.ManyOne));
  g.add_relation(
    binary_relation('Fav-Player', FAN, PLAYER, RelationKind.ManyOne)
  );
  g.add_relation(
    binary_relation('Fav-Color', FAN, COLOR, RelationKind.ManyOne)
  );

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
      ],
    };

    const g = new ERModel('Sport');

    g.add_entity(TEAM);
    g.add_entity(PLAYER);
    g.add_entity(FAN);
    g.add_entity(COLOR);

    g.add_relation(binary_relation('Team-Players', TEAM, PLAYER));
    g.add_relation(binary_relation('Team-Captain', TEAM, PLAYER));
    g.add_relation(binary_relation('Uniform-Colors', TEAM, COLOR));
    g.add_relation(
      binary_relation('Fav-Team', FAN, TEAM, RelationKind.ManyOne)
    );
    g.add_relation(
      binary_relation('Fav-Player', FAN, PLAYER, RelationKind.ManyOne)
    );
    g.add_relation(
      binary_relation('Fav-Color', FAN, COLOR, RelationKind.ManyOne)
    );
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

    g.add_relation(
      binary_relation('Is', captainship, PLAYER, RelationKind.ManyOne)
    );
    g.add_relation(binary_relation('Team-Players', TEAM, PLAYER));
    g.add_relation(binary_relation('Uniform-Colors', TEAM, COLOR));
    g.add_relation(
      binary_relation('Fav-Team', FAN, TEAM, RelationKind.ManyOne)
    );
    g.add_relation(
      binary_relation('Fav-Player', FAN, PLAYER, RelationKind.ManyOne)
    );
    g.add_relation(
      binary_relation('Fav-Color', FAN, COLOR, RelationKind.ManyOne)
    );
    g.add_relation(binary_relation('Led-By', PLAYER, captainship));
    g.add_relation(
      binary_relation('Team-Captain', TEAM, captainship, RelationKind.OneMany)
    );

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
    ],
  };

  const g = new ERModel('Sport');

  g.add_entity(TEAM);
  g.add_entity(PLAYER);
  g.add_entity(FAN);
  g.add_entity(COLOR);

  g.add_relation(
    binary_relation('Team-Players', TEAM, PLAYER, RelationKind.OneMany)
  );
  g.add_relation(
    binary_relation('Team-Captain', TEAM, PLAYER, RelationKind.OneOne)
  );
  g.add_relation(binary_relation('Uniform-Colors', TEAM, COLOR));
  g.add_relation(binary_relation('Fav-Team', FAN, TEAM, RelationKind.ManyOne));
  g.add_relation(
    binary_relation('Fav-Player', FAN, PLAYER, RelationKind.ManyOne)
  );
  g.add_relation(
    binary_relation('Fav-Color', FAN, COLOR, RelationKind.ManyOne)
  );
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
  ],
};

function exercise_4_1_6() {
  const g = new ERModel('People');

  g.add_entity(PEOPLE);

  const mother_of: Relation = {
    label: 'Mother-of',
    arrows: [
      [PEOPLE, Arrow.One, 'mother'],
      [PEOPLE, Arrow.Many, 'child'],
    ],
  };

  const father_of: Relation = {
    label: 'Father-of',
    arrows: [
      [PEOPLE, Arrow.One, 'father'],
      [PEOPLE, Arrow.Many, 'child'],
    ],
  };

  const child_of: Relation = {
    label: 'Child-of',
    arrows: [
      [PEOPLE, Arrow.Many, 'parent'],
      [PEOPLE, Arrow.Many, 'child'],
    ],
  };

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

  g.add_entity(PEOPLE);
  g.add_entity(female);
  g.add_entity(male);
  g.add_entity(father);
  g.add_entity(mother);

  g.add_isa(isa(PEOPLE, female));
  g.add_isa(isa(PEOPLE, male));
  g.add_isa(isa(female, mother));
  g.add_isa(isa(male, father));

  g.add_relation(
    binary_relation('Mother-of', mother, PEOPLE, RelationKind.OneMany)
  );
  g.add_relation(
    binary_relation('Father-of', father, PEOPLE, RelationKind.OneMany)
  );
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
      ],
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

    g.add_relation(
      binary_relation('Female-is', couple, PEOPLE, RelationKind.ManyOne)
    );
    g.add_relation(
      binary_relation('Male-is', couple, PEOPLE, RelationKind.ManyOne)
    );
    g.add_relation(
      binary_relation('Child-of', PEOPLE, couple, RelationKind.ManyOne)
    );

    g.output([OUTPUT_DIR, '4_1_8_b.png'].join('/'));
  }
}

function exercise_4_1_9() {
  const student = {
    label: 'Student',
    attrs: ['name', 'enrolled_year'],
  };

  const TA = {
    label: 'TA',
  };

  const department = {
    label: 'Department',
    attrs: ['name'],
  };

  const professor = {
    label: 'Professor',
    attrs: ['name'],
  };

  const course = {
    label: 'Course',
    attrs: ['name', 'year', 'is_remote'],
  };

  const enrolled_in: Relation = {
    label: 'Enrolled-in',
    attrs: ['score'],
    arrows: [
      [student, Arrow.Many],
      [course, Arrow.Many],
    ],
  };

  const g = new ERModel('University Registrar');

  g.add_entity(student);
  g.add_entity(department);
  g.add_entity(professor);
  g.add_entity(course);
  g.add_entity(TA);

  g.add_isa(isa(student, TA));

  g.add_relation(enrolled_in);
  // reasonable assumptions?
  g.add_relation(
    binary_relation('Teaching', professor, course, RelationKind.OneMany)
  );
  g.add_relation(
    binary_relation('Member-of', professor, department, RelationKind.ManyOne)
  );
  // some course may be jointly offered by multiple departments
  g.add_relation(binary_relation('Offer', department, course));
  g.add_relation(binary_relation('Assist', TA, course));
  // based on personal experience
  g.add_relation(
    binary_relation('Tutor-of', professor, student, RelationKind.OneMany)
  );

  g.output([OUTPUT_DIR, '4_1_9.png'].join('/'));
}

const STARS = {
  label: 'Stars',
};

const MOVIES = {
  label: 'Movies',
};

const STUDIOS = {
  label: 'Studios',
};

function exercise_4_1_10() {
  {
    const contract: Relation = {
      label: 'Contracts',
      arrows: [
        [STARS, Arrow.Many],
        [MOVIES, Arrow.Many],
        [STUDIOS, Arrow.One, 'Studio of Star'],
        [STARS, Arrow.Many, 'Producing Studio'],
      ],
    };

    const g = new ERModel('Movies');

    g.add_entity(STARS);
    g.add_entity(MOVIES);
    g.add_entity(STUDIOS);

    g.add_relation(contract);

    g.output([OUTPUT_DIR, '4_1_10_a.png'].join('/'));
  }

  {
    const contract: Relation = {
      label: 'Contracts',
      arrows: [
        [STARS, Arrow.Many],
        [MOVIES, Arrow.Many],
        [STUDIOS, Arrow.One],
      ],
    };

    const g = new ERModel('Movies');

    g.add_entity(STARS);
    g.add_entity(MOVIES);
    g.add_entity(STUDIOS);

    g.add_relation(contract);
    g.add_relation(
      binary_relation('Producing', STUDIOS, MOVIES, RelationKind.OneMany)
    );

    g.output([OUTPUT_DIR, '4_1_10_b.png'].join('/'));
  }
}

function exercise_4_2_1() {
  const customers = {
    label: 'Customers',
    attrs: ['name'],
  };

  const accounts = {
    label: 'Accounts',
    attrs: ['number', 'balance'],
  };

  const addresses = {
    label: 'Addresses',
    attrs: ['address'],
  };

  const g = new ERModel('Bank');

  g.add_entity(customers);
  g.add_entity(accounts);
  g.add_entity(addresses);

  g.add_relation(
    binary_relation('Lives-at', customers, addresses, RelationKind.ManyOne)
  );
  g.add_relation(binary_relation('Owns', customers, accounts));

  g.output([OUTPUT_DIR, '4_2_1.png'].join('/'));
}

function exercise_4_2_3() {
  const contracts: Relation = {
    label: 'Contracts',
    attrs: ['salary', 'studio_name'],
    arrows: [
      [MOVIES, Arrow.Many],
      [STARS, Arrow.Many],
    ],
  };

  const g = new ERModel('Movies');

  g.add_entity(MOVIES);
  g.add_entity(STARS);

  g.add_relation(contracts);

  g.output([OUTPUT_DIR, '4_2_3.png'].join('/'));
}

const MOTHERS = {
  label: 'Mothers',
};

const BABIES = {
  label: 'Babies',
};

const DOCTORS = {
  label: 'Doctors',
};

const NURSES = {
  label: 'Nurses',
};

function exercise_4_2_5() {
  const births: Relation = {
    label: 'Births',
    arrows: [
      [MOTHERS, Arrow.One],
      [BABIES, Arrow.Many],
      [DOCTORS, Arrow.One],
      [NURSES, Arrow.Many],
    ],
  };

  const g = new ERModel('Birth');

  g.add_entity(MOTHERS);
  g.add_entity(BABIES);
  g.add_entity(DOCTORS);
  g.add_entity(NURSES);

  g.add_relation(births);
  g.add_relation(
    binary_relation('Mother-of', MOTHERS, BABIES, RelationKind.OneMany)
  );
  g.add_relation(
    binary_relation('Midwifed', DOCTORS, BABIES, RelationKind.OneMany)
  );

  g.output([OUTPUT_DIR, '4_2_5.png'].join('/'));
}

function exercise_4_2_6() {
  const births = {
    label: 'Births',
  };

  const g = new ERModel('Birth');

  g.add_entity(MOTHERS);
  g.add_entity(BABIES);
  g.add_entity(DOCTORS);
  g.add_entity(NURSES);
  g.add_entity(births);

  g.add_relation(
    binary_relation('Birth-of', births, BABIES, RelationKind.OneOne)
  );
  g.add_relation(
    binary_relation('Given-by', births, MOTHERS, RelationKind.ManyOne)
  );
  g.add_relation(
    binary_relation('Midwifed-by', births, DOCTORS, RelationKind.ManyOne)
  );
  g.add_relation(binary_relation('Assisted-by', births, NURSES));

  g.output([OUTPUT_DIR, '4_2_6.png'].join('/'));
}

function exercise_4_2_7() {
  const births = {
    label: 'Births',
  };

  const g = new ERModel('Birth');

  g.add_entity(MOTHERS);
  g.add_entity(BABIES);
  g.add_entity(DOCTORS);
  g.add_entity(NURSES);
  g.add_entity(births);

  g.add_relation(
    binary_relation('Birth-of', births, BABIES, RelationKind.OneMany)
  );
  g.add_relation(
    binary_relation('Given-by', births, MOTHERS, RelationKind.ManyOne)
  );
  g.add_relation(
    binary_relation('Midwifed-by', births, DOCTORS, RelationKind.ManyOne)
  );
  g.add_relation(binary_relation('Assisted-by', births, NURSES));

  g.output([OUTPUT_DIR, '4_2_7.png'].join('/'));
}

function exercise_4_3_1() {
  {
    const customer = {
      keys: ['ssn'],
      ...CUSTOMER,
    };
    const account = {
      keys: ['number'],
      ...ACCOUNT,
    };

    const own: Relation = {
      label: 'Own',
      arrows: [
        [customer, Arrow.RI],
        [account, Arrow.Many],
      ],
    };

    const g = new ERModel('Bank');

    g.add_entity(customer);
    g.add_entity(account);

    g.add_relation(own);

    g.output([OUTPUT_DIR, '4_3_1_a.png'].join('/'));
  }

  {
    const team = {
      keys: ['name'],
      ...TEAM,
    };
    const player = {
      keys: ['name'],
      ...PLAYER,
    };
    const fan = {
      keys: ['name'],
      ...FAN,
    };
    const color = {
      keys: ['name'],
      ...COLOR,
    };

    const g = new ERModel('Sport');

    g.add_entity(team);
    g.add_entity(player);
    g.add_entity(fan);
    g.add_entity(color);

    g.add_relation(
      binary_relation('Team-Players', team, player, RelationKind.OneMany)
    );
    g.add_relation(
      binary_relation('Team-Captain', team, player, RelationKind.OneOne)
    );
    g.add_relation(binary_relation('Uniform-Colors', team, color));
    g.add_relation(
      binary_relation('Fav-Team', fan, team, RelationKind.ManyOne)
    );
    g.add_relation(
      binary_relation('Fav-Player', fan, player, RelationKind.ManyOne)
    );
    g.add_relation(
      binary_relation('Fav-Color', fan, color, RelationKind.ManyOne)
    );

    g.output([OUTPUT_DIR, '4_3_1_b.png'].join('/'));
  }

  {
    const people = {
      keys: ['name'],
      ...PEOPLE,
    };

    const g = new ERModel('People');

    g.add_entity(people);

    const mother_of: Relation = {
      label: 'Mother-of',
      arrows: [
        [people, Arrow.RI, 'mother'],
        [people, Arrow.Many, 'child'],
      ],
    };

    const father_of: Relation = {
      label: 'Father-of',
      arrows: [
        [people, Arrow.RI, 'father'],
        [people, Arrow.Many, 'child'],
      ],
    };

    const child_of: Relation = {
      label: 'Child-of',
      arrows: [
        [people, Arrow.Many, 'parent'],
        [people, Arrow.Many, 'child'],
      ],
    };

    g.add_relation(mother_of);
    g.add_relation(father_of);
    g.add_relation(child_of);

    g.output([OUTPUT_DIR, '4_3_1_c.png'].join('/'));
  }
}

const STUDENT = {
  label: 'Student',
  attrs: ['id'],
  keys: ['id'],
};

const COURSE = {
  label: 'Course',
  attrs: ['id'],
  keys: ['id'],
};

function exercise_4_4_1() {
  const enrollment = {
    label: 'Enrollment',
    attrs: ['score'],
    is_weak: true,
  };

  const g = new ERModel('Enrollment');

  g.add_entity(STUDENT);
  g.add_entity(COURSE);
  g.add_entity(enrollment);

  g.add_relation(support_relation('Student-of', enrollment, STUDENT));
  g.add_relation(support_relation('Course-of', enrollment, COURSE));

  g.output([OUTPUT_DIR, '4_4_1.png'].join('/'));
}

function exercise_4_4_2() {
  const enrollment = {
    label: 'Enrollment',
    is_weak: true,
  };

  const assignment = {
    label: 'Assignment',
    attrs: ['score', 'id'],
    keys: ['id'],
    is_weak: true,
  };

  const g = new ERModel('Enrollment');

  g.add_entity(STUDENT);
  g.add_entity(COURSE);
  g.add_entity(enrollment);
  g.add_entity(assignment);

  g.add_relation(support_relation('Student-of', enrollment, STUDENT));
  g.add_relation(support_relation('Course-of', enrollment, COURSE));
  g.add_relation(support_relation('Enrollment-of', assignment, enrollment));

  g.output([OUTPUT_DIR, '4_4_2.png'].join('/'));
}

function exercise_4_4_3() {
  const births = {
    label: 'Births',
    is_weak: true,
  };

  const birth_of: Relation = {
    label: 'Birth-of',
    arrows: [
      [births, Arrow.One],
      [BABIES, Arrow.RI],
    ],
    is_support: true,
  };

  const g = new ERModel('Birth');

  g.add_entity(MOTHERS);
  g.add_entity(BABIES);
  g.add_entity(DOCTORS);
  g.add_entity(NURSES);
  g.add_entity(births);

  g.add_relation(birth_of);
  g.add_relation(
    binary_relation('Given-by', births, MOTHERS, RelationKind.ManyOne)
  );
  g.add_relation(
    binary_relation('Midwifed-by', births, DOCTORS, RelationKind.ManyOne)
  );
  g.add_relation(binary_relation('Assisted-by', births, NURSES));

  g.output([OUTPUT_DIR, '4_4_3.png'].join('/'));
}

function exercise_4_4_4() {
  {
    const course = {
      label: 'Course',
      attrs: ['number'],
      keys: ['number'],
      is_weak: true,
    };

    const department = {
      label: 'Department',
      attrs: ['name'],
      keys: ['name'],
    };

    const g = new ERModel('University');

    g.add_entity(course);
    g.add_entity(department);

    g.add_relation(support_relation('Offered-by', course, department));
    g.output([OUTPUT_DIR, '4_4_4_a.png'].join('/'));
  }

  {
    const leagues = {
      label: 'Leagues',
      attrs: ['names'],
      keys: ['names'],
    };

    const teams = {
      label: 'Teams',
      attrs: ['names'],
      keys: ['names'],
      is_weak: true,
    };

    const players = {
      label: 'Players',
      attrs: ['number'],
      keys: ['number'],
      is_weak: true,
    };

    const g = new ERModel('Leagues');

    g.add_entity(leagues);
    g.add_entity(teams);
    g.add_entity(players);

    g.add_relation(support_relation('In-League', teams, leagues));
    g.add_relation(support_relation('In-Team', players, teams));

    g.output([OUTPUT_DIR, '4_4_4_b.png'].join('/'));
  }
}

function exercise_4_5_2() {
  const customers = {
    label: 'Customers',
    attrs: ['SSNo', 'name', 'addr', 'phone'],
    keys: ['SSNo'],
  };

  const flights = {
    label: 'Flights',
    attrs: ['number', 'day', 'aircraft'],
    keys: ['number', 'day'],
  };

  const bookings = {
    label: 'Bookings',
    attrs: ['row', 'seat'],
    keys: ['row', 'seat'],
    is_weak: true,
  };

  const to_cust: Relation = {
    label: 'toCust',
    arrows: [
      [bookings, Arrow.Many],
      [customers, Arrow.RI],
    ],
  };

  const g = new ERModel('Airlines');

  g.add_entity(customers);
  g.add_entity(flights);
  g.add_entity(bookings);

  g.add_relation(support_relation('toFlt', bookings, flights));
  g.add_relation(to_cust);

  g.output([OUTPUT_DIR, '4_5_2.png'].join('/'));
}

function exercise_4_7_1() {
  const g = new UML('Bank');

  const customer = {
    ...CUSTOMER,
    keys: ['ssn'],
  };

  const account = {
    ...ACCOUNT,
    keys: ['number'],
  };

  g.add_class(customer);
  g.add_class(account);
  g.add_association(association('Own', customer, account));

  g.output([OUTPUT_DIR, '4_7_1.png'].join('/'));
}

function exercise_4_7_2() {
  {
    const g = new UML('Bank');
    g.add_class(CUSTOMER);
    g.add_class(ACCOUNT);
    g.add_association(
      association('Own', CUSTOMER, ACCOUNT, RelationKind.ManyOne)
    );

    g.output([OUTPUT_DIR, '4_7_2_a.png'].join('/'));
  }

  {
    const g = new UML('Bank');
    g.add_class(CUSTOMER);
    g.add_class(ACCOUNT);
    g.add_association(
      association('Own', CUSTOMER, ACCOUNT, RelationKind.OneOne)
    );

    g.output([OUTPUT_DIR, '4_7_2_b.png'].join('/'));
  }

  {
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

    const g = new UML('Bank');

    g.add_class(customer);
    g.add_class(phone);
    g.add_class(address);
    g.add_class(ACCOUNT);

    g.add_association(association('Own-Account', customer, ACCOUNT));
    g.add_association(association('Own-Phone', customer, phone));
    g.add_association(
      association('Live-in', customer, address, RelationKind.ManyOne)
    );

    g.output([OUTPUT_DIR, '4_7_2_c.png'].join('/'));
  }

  {
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

    const g = new UML('Bank');

    g.add_class(customer);
    g.add_class(phone);
    g.add_class(address);
    g.add_class(ACCOUNT);

    g.add_association(association('Own-Account', customer, ACCOUNT));
    g.add_association(association('Has-Phone', address, phone));
    g.add_association(
      association('Live-In', customer, address, RelationKind.ManyOne)
    );

    g.output([OUTPUT_DIR, '4_7_2_d.png'].join('/'));
  }
}

function exercise_4_7_3() {
  const g = new UML('Sport');

  g.add_class(TEAM);
  g.add_class(PLAYER);
  g.add_class(FAN);
  g.add_class(COLOR);

  g.add_association(association('TeamPlayers', TEAM, PLAYER));
  g.add_association(
    association('Team-Captain', TEAM, PLAYER, RelationKind.OneOne)
  );
  g.add_association(association('Uniform-Colors', TEAM, COLOR));
  g.add_association(association('Fav-Team', FAN, TEAM, RelationKind.ManyOne));
  g.add_association(
    association('Fav-Player', FAN, PLAYER, RelationKind.ManyOne)
  );
  g.add_association(association('Fav-Color', FAN, COLOR, RelationKind.ManyOne));

  g.output([OUTPUT_DIR, '4_7_3.png'].join('/'));
}

function exercise_4_7_4() {
  const g = new UML('People');

  g.add_class(PEOPLE);

  const mother_of: Association = {
    label: 'Mother-of',
    from: [PEOPLE, mul(0, 1), 'mother'],
    to: [PEOPLE, mul(), 'child'],
  };

  const father_of: Association = {
    label: 'Father-of',
    from: [PEOPLE, mul(0, 1), 'father'],
    to: [PEOPLE, mul(), 'child'],
  };

  const child_of: Association = {
    label: 'Child-of',
    from: [PEOPLE, mul(), 'parent'],
    to: [PEOPLE, mul(), 'child'],
  };

  g.add_association(mother_of);
  g.add_association(father_of);
  g.add_association(child_of);

  g.output([OUTPUT_DIR, '4_7_4.png'].join('/'));
}

function exercise_4_7_5() {
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

  const g = new UML('People');

  g.add_entity(PEOPLE);
  g.add_entity(female);
  g.add_entity(male);
  g.add_entity(father);
  g.add_entity(mother);

  g.add_isa(isa(PEOPLE, female));
  g.add_isa(isa(PEOPLE, male));
  g.add_isa(isa(female, mother));
  g.add_isa(isa(male, father));

  g.add_relation(
    association('Mother-of', mother, PEOPLE, RelationKind.OneMany)
  );
  g.add_relation(
    association('Father-of', father, PEOPLE, RelationKind.OneMany)
  );
  g.add_relation(association_from_self_relation(CHILD_OF));

  g.output([OUTPUT_DIR, '4_7_5.png'].join('/'));
}

function exercise_4_7_6() {
  const student = {
    label: 'Student',
    attrs: ['name', 'enrolled_year'],
  };

  const TA = {
    label: 'TA',
  };

  const department = {
    label: 'Department',
    attrs: ['name'],
  };

  const professor = {
    label: 'Professor',
    attrs: ['name'],
  };

  const course = {
    label: 'Course',
    attrs: ['name', 'year', 'is_remote'],
  };

  const enrolled_in: Association = {
    ...association('Enrolled-In', student, course),
    class: {
      label: 'Enrollment',
      attrs: ['score'],
    },
  };

  const g = new UML('University Registrar');

  g.add_entity(student);
  g.add_entity(department);
  g.add_entity(professor);
  g.add_entity(course);
  g.add_entity(TA);

  g.add_isa(isa(student, TA));

  g.add_relation(enrolled_in);
  // reasonable assumptions?
  g.add_relation(
    association('Teaching', professor, course, RelationKind.OneMany)
  );
  g.add_relation(
    association('Member-of', professor, department, RelationKind.ManyOne)
  );
  // some course may be jointly offered by multiple departments
  g.add_relation(association('Offer', department, course));
  g.add_relation(association('Assist', TA, course));
  // based on personal experience
  g.add_relation(
    association('Tutor-of', professor, student, RelationKind.OneMany)
  );

  g.output([OUTPUT_DIR, '4_7_6.png'].join('/'));
}

function exercise_4_7_7() {
  const ships = {
    label: 'Ships',
    keys: ['name'],
    attrs: ['name', 'yearLaunched'],
  };

  const sister_of: Association = {
    label: 'Sister-of',
    from: [ships, mul(), 'TheSister'],
    to: [ships, mul(), 'TheShip'],
  };

  const g = new UML('Ships');

  g.add_class(ships);
  g.add_relation(sister_of);

  g.output([OUTPUT_DIR, '4_7_7.png'].join('/'));
}

function exercise_4_7_9() {
  const births = {
    label: 'Births',
  };

  const g = new UML('Birth');

  g.add_entity(MOTHERS);
  g.add_entity(BABIES);
  g.add_entity(DOCTORS);
  g.add_entity(NURSES);
  g.add_class(births);

  g.add_association({
    label: 'BirthOf',
    from: [births, mul(1, 1)],
    to: [BABIES, mul(1, 1)],
  });

  g.add_association({
    label: 'MotherOf',
    from: [MOTHERS, mul(1, 1)],
    to: [BABIES, mul()],
  });

  g.add_association({
    label: 'Midwifed',
    from: [births, mul()],
    to: [DOCTORS, mul(1, 1)],
  });

  g.add_association(association('Assisted', NURSES, births));

  g.output([OUTPUT_DIR, '4_7_9.png'].join('/'));
}

function ER_exercises() {
  exercise_4_1_1();
  exercise_4_1_2();
  exercise_4_1_3();
  exercise_4_1_4();
  exercise_4_1_5();
  exercise_4_1_6();
  exercise_4_1_7();
  exercise_4_1_8();
  exercise_4_1_9();
  exercise_4_1_10();
  exercise_4_2_1();
  exercise_4_2_3();
  exercise_4_2_5();
  exercise_4_2_6();
  exercise_4_2_7();
  exercise_4_3_1();
  exercise_4_4_1();
  exercise_4_4_2();
  exercise_4_4_3();
  exercise_4_4_4();
  exercise_4_5_2();
}

// low effort, low quality
function UML_exercises() {
  exercise_4_7_1();
  exercise_4_7_2();
  exercise_4_7_3();
  // when there's too much self associations the output looks horrible
  exercise_4_7_4();
  exercise_4_7_5();
  exercise_4_7_6();
  exercise_4_7_7();
  exercise_4_7_9();
}

function ODL_exercises() {
  validate(`
class Customer (key (ssn)) {
    attribute integer ssn;
    attribute string name;
    attribute string address;
    attribute integer phone;
    relationship Set<Account> owns inverse Account::ownedBy;
};
class Account (key (number)) {
    attribute integer number;
    // in cents
    attribute integer balance; 
    // what is a type exactly?
    attribute enum Type { type } type;
    relationship Set<Customer> ownedBy inverse Customer::owns;
};
  `);

  validate(`
class Team (key (name)) {
    attribute string name;
    // attributes in ODL can be sets of primitive types
    // colors can be an attribute instead of a class
    attribute Set<string> uniformColors;
    relationship Set<Player> players inverse Player::team;
    relationship Player captain inverse Player::captainOf;
};
class Player (key (name)) {
    attribute string name;
    relationship Team team inverse Team::players; 
    relationship Team captionOf inverse Team::captain;
};
class Fan (key (name)) {
    // these relations are not mutual: they hold no significance on the other end
    attribute Team favTeam;
    attribute Player favPlayer;
    attribute string favColor;
};
  `);

  validate(`
class Person {
    attribute string name;
    relationship Set<Person> parents inverse Person::children;
    relationship Set<Person> children inverse Person::parents;
    // inverse of these two are subsets of children, but not exactly children
    relationship Person mother;
    relationship Person father;
};
  `);

  validate(`
class Degree {
    attribute string name;
    attribute string school;
    attribute string date;
};
  `);

  validate(`
    class Department (key (name)) {
        attribute string name;
        relationship Set<Course> courses inverse Course::department;
    };
    class Course (key (number, department)) {
        attribute integer number;
        relationship Department department inverse Department::courses;
    };
  `);

  validate(`
    class League (key (name)) {
        attribute string name;
        relationship Set<Team> teams inverse Team::league;
        relationship Set<Player> players inverse Player::league;
    };
    class Team (key (name, league)) {
        attribute string name;
        relationship League league inverse League::teams;
        relationship Set<Player> players inverse Player::team;
    };
    class Player (key (name, team, league)) {
        attribute string name;
        relationship League league inverse League::players;
        relationship Team team inverse Team::players;
    };
  `);

  validate(`
class Department (key (name)) {
    attribute string name;
    relationship Set<Course> courses inverse Course::offedBy;
    relationship Set<Professor> members inverse Professor::memberOf;
};
class Professor (key (name)) {
    attribute string name;
    relationship Department memberOf inverse Department::members;
    relationship Set<Course> teaches inverse Course::taughtBy;
    relationship Set<Student> tutorOf inverse Student::tutor;
};
class Course (key (name, year)) {
    attribute string name;
    attribute integer year;
    attribute boolean is_remote;
    relationship Department offedBy inverse Department::courses;
    relationship Professor taughtBy inverse Professor::teaches;
    // integer here is the score
    relationship Dictionary<Student, integer> students inverse Student::enrolledIn;
    relationship Set<TA> ta inverse TA::assists;
};
class Student (key (name, enrolledYear)) {
    attribute string name; 
    attribute integer enrolledYear;
    relationship Professor tutor inverse Professor::tutorOf;
    // integer here is the score
    relationship Dictionary<Course, integer> enrolledIn inverse Course::students;
};
class TA extends Student {
    relationship Set<Course> assists inverse Course::ta;
};
  `);
}

function main() {
  if (!fs.existsSync(OUTPUT_DIR)) {
    fs.mkdirSync(OUTPUT_DIR);
  }

  switch (process.argv[2]?.toUpperCase()) {
    case 'ER':
      ER_exercises();
      break;
    case 'UML':
      UML_exercises();
      break;
    case 'ODL':
      ODL_exercises();
      break;
    default:
      ER_exercises();
      UML_exercises();
      ODL_exercises();
  }
}

main();

#![allow(dead_code)]

use std::fmt;
use std::mem;
use std::str;
use std::u8;

#[derive(Clone, Copy, Debug)]
enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let register_name = format!("{:?}", self).to_lowercase();
        write!(f, "%{}", register_name);
        Ok(())
    }
}

use self::Register::*;

fn register_id(or: Option<Register>) -> u8 {
    if let Some(r) = or {
        r as u8
    } else {
        0xF
    }
}

fn id_register(id: u8) -> Option<Register> {
    assert!(id <= 0xF);
    match id {
        0x0 => Some(EAX),
        0x1 => Some(ECX),
        0x2 => Some(EDX),
        0x3 => Some(EBX),
        0x4 => Some(ESP),
        0x5 => Some(EBP),
        0x6 => Some(ESI),
        0x7 => Some(EDI),
        _ => None,
    }
}

fn u4_pair(a: u8, b: u8) -> u8 {
    (a << 4) | b
}

fn u4_unpair(a: u8) -> (u8, u8) {
    (a >> 4, a & 0xf)
}

fn register_pair(ora: Option<Register>, orb: Option<Register>) -> u8 {
    u4_pair(register_id(ora), register_id(orb))
}

type Addr = i32;
type Imm = i32;

fn le(v: i32) -> [u8; 4] {
    unsafe { mem::transmute(v.to_le()) }
}

fn from_le(input: &[u8]) -> i32 {
    let mut bytes: [u8; 4] = [0; 4];
    for i in 0..4 {
        bytes[i] = input[i];
    }
    unsafe {
        let le: i32 = mem::transmute(bytes);
        i32::from_le(le)
    }
}

#[derive(Clone, Copy, Debug)]
enum Condition {
    LE = 0x1,
    L = 0x2,
    E = 0x3,
    NE = 0x4,
    GE = 0x5,
    G = 0x6,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let cond_name = format!("{:?}", self).to_lowercase();
        write!(f, "{}", cond_name);
        Ok(())
    }
}

use self::Condition::*;

fn condition_id(oc: Option<Condition>) -> u8 {
    if let Some(c) = oc {
        c as u8
    } else {
        0x0
    }
}

fn valid_condition(id: u8) -> bool {
    id <= 0x6
}

fn id_condition(id: u8) -> Option<Condition> {
    assert!(valid_condition(id));
    match id {
        0x1 => Some(LE),
        0x2 => Some(L),
        0x3 => Some(E),
        0x4 => Some(NE),
        0x5 => Some(GE),
        0x6 => Some(G),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    ADD,
    SUB,
    AND,
    XOR,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let op_name = format!("{:?}", self).to_lowercase();
        write!(f, "{}", op_name);
        Ok(())
    }
}

fn op_id(op: Op) -> u8 {
    op as u8
}

fn valid_op(id: u8) -> bool {
    id <= 0x3
}

fn id_op(id: u8) -> Op {
    assert!(valid_op(id));
    match id {
        0x0 => ADD,
        0x1 => SUB,
        0x2 => AND,
        0x3 => XOR,
        _ => unreachable!(),
    }
}
use self::Op::*;

#[derive(Clone, Copy)]
enum Spec {
    HALT,
    NOP,
    CMOV,
    IRMOVL,
    RMMOVL,
    MRMOVL,
    OPL,
    JMP,
    CALL,
    RET,
    PUSHL,
    POPL,
}

impl Spec {
    fn size(self) -> usize {
        use self::Spec::*;
        match self {
            HALT => 1,
            NOP => 1,
            CMOV => 6,
            IRMOVL => 6,
            RMMOVL => 6,
            MRMOVL => 6,
            OPL => 2,
            JMP => 5,
            CALL => 5,
            RET => 1,
            PUSHL => 2,
            POPL => 2,
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    HALT,
    NOP,
    RRMOVL(Register, Register),
    IRMOVL(Imm, Register),
    RMMOVL(Register, Imm, Register),
    MRMOVL(Imm, Register, Register),
    OPL(Op, Register, Register),
    JMP(Option<Condition>, Addr),
    CMOV(Condition, Register, Register),
    CALL(Addr),
    RET,
    PUSHL(Register),
    POPL(Register),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::Instruction::*;
        match *self {
            HALT => write!(f, "halt"),
            NOP => write!(f, "nop"),
            RRMOVL(ra, rb) => write!(f, "rrmovl\t{}, {}", ra, rb),
            IRMOVL(imm, rb) => write!(f, "irmovl\t${}, {}", imm, rb),
            RMMOVL(ra, diff, rb) => write!(f, "rmmovl\t{}, {}({})", ra, diff, rb),
            MRMOVL(diff, ra, rb) => write!(f, "mrmovl\t{}({}), {}", diff, ra, rb),
            OPL(op, ra, rb) => write!(f, "{}l\t{}, {}", op, ra, rb),
            JMP(oc, addr) => if let Some(cond) = oc {
                write!(f, "j{}\t0x{:X}", cond, addr)
            } else {
                write!(f, "jmp\t0x{:X}", addr)
            },
            CMOV(cond, ra, rb) => write!(f, "cmov{}\t{}, {}", cond, ra, rb),
            CALL(addr) => write!(f, "call\t0x{:X}", addr),
            RET => write!(f, "ret"),
            PUSHL(ra) => write!(f, "pushl\t{}", ra),
            POPL(ra) => write!(f, "popl\t{}", ra),
        }
    }
}

impl Instruction {
    fn encode(self) -> Vec<u8> {
        use self::Instruction::*;
        let mut code: Vec<u8> = vec![];
        match self {
            HALT => code.push(u4_pair(Spec::HALT as u8, 0x0)),
            NOP => code.push(u4_pair(Spec::NOP as u8, 0x0)),
            RRMOVL(ra, rb) => {
                code.push(u4_pair(Spec::CMOV as u8, 0x0));
                code.push(register_pair(Some(ra), Some(rb)));
            }
            IRMOVL(v, rb) => {
                code.push(u4_pair(Spec::IRMOVL as u8, 0x0));
                code.push(register_pair(None, Some(rb)));
                code.extend_from_slice(&le(v));
            }
            RMMOVL(ra, d, rb) => {
                code.push(u4_pair(Spec::RMMOVL as u8, 0x0));
                code.push(register_pair(Some(ra), Some(rb)));
                code.extend_from_slice(&le(d));
            }
            MRMOVL(d, ra, rb) => {
                code.push(u4_pair(Spec::MRMOVL as u8, 0x0));
                code.push(register_pair(Some(ra), Some(rb)));
                code.extend_from_slice(&le(d));
            }
            OPL(op, ra, rb) => {
                code.push(u4_pair(Spec::OPL as u8, op_id(op)));
                code.push(register_pair(Some(ra), Some(rb)));
            }
            JMP(oc, addr) => {
                code.push(u4_pair(Spec::JMP as u8, condition_id(oc)));
                code.extend_from_slice(&le(addr));
            }
            CMOV(c, ra, rb) => {
                code.push(u4_pair(Spec::CMOV as u8, condition_id(Some(c))));
                code.push(register_pair(Some(ra), Some(rb)));
            }
            CALL(addr) => {
                code.push(u4_pair(Spec::CALL as u8, 0x0));
                code.extend_from_slice(&le(addr));
            }
            RET => code.push(u4_pair(Spec::RET as u8, 0x0)),
            PUSHL(ra) => {
                code.push(u4_pair(Spec::PUSHL as u8, 0x0));
                code.push(register_pair(Some(ra), None));
            }
            POPL(ra) => {
                code.push(u4_pair(Spec::POPL as u8, 0x0));
                code.push(register_pair(Some(ra), None));
            }
        };

        code
    }
}

fn encode_instructions(instrs: Vec<Instruction>) -> Vec<u8> {
    let mut code: Vec<u8> = vec![];
    for instr in instrs {
        code.extend_from_slice(&instr.encode());
    }
    code
}

#[derive(Debug)]
enum DecodeError {
    UnexpectedEof,
    InvalidFnCode,
    InvalidInstSpec,
    InvalidRegisterCode,
}

use self::DecodeError::*;

struct InstDecoder {
    pos: usize,
    input: Vec<u8>,
}

impl InstDecoder {
    fn new(input: Vec<u8>) -> Self {
        InstDecoder { pos: 0, input }
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn remaining(&self) -> usize {
        if self.eof() {
            0
        } else {
            self.input.len() - self.pos
        }
    }

    fn next_byte(&self) -> u8 {
        self.input[self.pos]
    }

    fn consume_byte(&mut self) -> Result<u8, DecodeError> {
        if self.eof() {
            return Err(UnexpectedEof);
        }
        let c = self.input[self.pos];
        self.pos += 1;
        Ok(c)
    }

    fn parse_u4_pair(&mut self) -> Result<(u8, u8), DecodeError> {
        let c = self.consume_byte()?;
        Ok(u4_unpair(c))
    }

    fn parse_registers(&mut self) -> Result<(Option<Register>, Option<Register>), DecodeError> {
        let (ra, rb) = self.parse_u4_pair()?;
        if ra > 0xf || rb > 0xf {
            return Err(InvalidRegisterCode);
        }
        Ok((id_register(ra), id_register(rb)))
    }

    fn parse_imm(&mut self) -> Result<Imm, DecodeError> {
        if self.remaining() < 4 {
            return Err(UnexpectedEof);
        }
        let imm = from_le(&self.input[self.pos..]);
        self.pos += 4;
        Ok(imm)
    }

    fn parse_single(&mut self, instr: Instruction) -> Result<Instruction, DecodeError> {
        self.parse_instr(
            |fncode| fncode == 0x0,
            false,
            false,
            |_, _, _, _| Some(instr),
        )
    }

    fn parse_instr<V, F>(
        &mut self,
        fn_validation: V,
        has_imm: bool,
        has_rgst: bool,
        constr: F,
    ) -> Result<Instruction, DecodeError>
    where
        V: Fn(u8) -> bool,
        F: Fn(Imm, u8, Option<Register>, Option<Register>) -> Option<Instruction>,
    {
        let (_, fncode) = self.parse_u4_pair()?;
        if !fn_validation(fncode) {
            return Err(InvalidFnCode);
        }
        let (ora, orb) = if has_rgst {
            self.parse_registers()?
        } else {
            (None, None)
        };
        let imm = if has_imm { self.parse_imm()? } else { 0 };
        if let Some(instr) = constr(imm, fncode, ora, orb) {
            Ok(instr)
        } else {
            Err(InvalidRegisterCode)
        }
    }

    fn parse_mov<V, F>(
        &mut self,
        fn_validation: V,
        has_imm: bool,
        constr: F,
    ) -> Result<Instruction, DecodeError>
    where
        V: Fn(u8) -> bool,
        F: Fn(Imm, u8, Option<Register>, Option<Register>) -> Option<Instruction>,
    {
        self.parse_instr(fn_validation, has_imm, true, constr)
    }

    fn parse_double<V, F>(
        &mut self,
        fn_validation: V,
        constr: F,
    ) -> Result<Instruction, DecodeError>
    where
        V: Fn(u8) -> bool,
        F: Fn(u8, Option<Register>, Option<Register>) -> Option<Instruction>,
    {
        self.parse_mov(fn_validation, false, |_, fncode, ora, orb| {
            constr(fncode, ora, orb)
        })
    }

    fn parse_calljmp<V, F>(
        &mut self,
        fn_validation: V,
        constr: F,
    ) -> Result<Instruction, DecodeError>
    where
        V: Fn(u8) -> bool,
        F: Fn(Imm, u8) -> Instruction,
    {
        self.parse_instr(fn_validation, true, false, |imm, fncode, _, _| {
            Some(constr(imm, fncode))
        })
    }

    fn parse_halt(&mut self) -> Result<Instruction, DecodeError> {
        self.parse_single(Instruction::HALT)
    }

    fn parse_nop(&mut self) -> Result<Instruction, DecodeError> {
        self.parse_single(Instruction::NOP)
    }

    fn parse_cmov(&mut self) -> Result<Instruction, DecodeError> {
        use self::Instruction::*;

        fn constr(fncode: u8, ora: Option<Register>, orb: Option<Register>) -> Option<Instruction> {
            match (ora, orb) {
                (Some(ra), Some(rb)) => {
                    if let Some(cond) = id_condition(fncode) {
                        Some(CMOV(cond, ra, rb))
                    } else {
                        Some(RRMOVL(ra, rb))
                    }
                }
                _ => None,
            }
        }

        self.parse_double(valid_condition, constr)
    }

    fn parse_irmovl(&mut self) -> Result<Instruction, DecodeError> {
        use self::Instruction::*;

        self.parse_mov(
            |fncode| fncode == 0x0,
            true,
            |imm, _, ora, orb| match (ora, orb) {
                (None, Some(rb)) => Some(IRMOVL(imm, rb)),
                _ => None,
            },
        )
    }

    fn parse_rmmovl(&mut self) -> Result<Instruction, DecodeError> {
        use self::Instruction::*;

        self.parse_mov(
            |fncode| fncode == 0x0,
            true,
            |diff, _, ora, orb| match (ora, orb) {
                (Some(ra), Some(rb)) => Some(RMMOVL(ra, diff, rb)),
                _ => None,
            },
        )
    }

    fn parse_mrmovl(&mut self) -> Result<Instruction, DecodeError> {
        use self::Instruction::*;

        self.parse_mov(
            |fncode| fncode == 0x0,
            true,
            |diff, _, ora, orb| match (ora, orb) {
                (Some(ra), Some(rb)) => Some(MRMOVL(diff, ra, rb)),
                _ => None,
            },
        )
    }

    fn parse_opl(&mut self) -> Result<Instruction, DecodeError> {
        use self::Instruction::*;

        self.parse_double(valid_op, |fncode, ora, orb| match (ora, orb) {
            (Some(ra), Some(rb)) => {
                let op = id_op(fncode);
                Some(OPL(op, ra, rb))
            }
            _ => None,
        })
    }

    fn parse_jmp(&mut self) -> Result<Instruction, DecodeError> {
        self.parse_calljmp(valid_condition, |addr, fncode| {
            let cond = id_condition(fncode);
            Instruction::JMP(cond, addr)
        })
    }

    fn parse_call(&mut self) -> Result<Instruction, DecodeError> {
        self.parse_calljmp(|fncode| fncode == 0x0, |addr, _| Instruction::CALL(addr))
    }

    fn parse_ret(&mut self) -> Result<Instruction, DecodeError> {
        self.parse_single(Instruction::RET)
    }

    fn parse_pushl(&mut self) -> Result<Instruction, DecodeError> {
        self.parse_double(
            |fncode| fncode == 0x0,
            |_, ora, orb| match (ora, orb) {
                (Some(ra), None) => Some(Instruction::PUSHL(ra)),
                _ => None,
            },
        )
    }

    fn parse_popl(&mut self) -> Result<Instruction, DecodeError> {
        self.parse_double(
            |fncode| fncode == 0x0,
            |_, ora, orb| match (ora, orb) {
                (Some(ra), None) => Some(Instruction::POPL(ra)),
                _ => None,
            },
        )
    }

    fn parse_instruction(&mut self) -> Result<Instruction, DecodeError> {
        let byte = self.next_byte();
        let (spec, _) = u4_unpair(byte);
        match spec {
            0x0 => self.parse_halt(),
            0x1 => self.parse_nop(),
            0x2 => self.parse_cmov(),
            0x3 => self.parse_irmovl(),
            0x4 => self.parse_rmmovl(),
            0x5 => self.parse_mrmovl(),
            0x6 => self.parse_opl(),
            0x7 => self.parse_jmp(),
            0x8 => self.parse_call(),
            0x9 => self.parse_ret(),
            0xa => self.parse_pushl(),
            0xb => self.parse_popl(),
            _ => Err(InvalidInstSpec),
        }
    }

    fn decode(&mut self) -> Result<Vec<Instruction>, DecodeError> {
        let mut instrs: Vec<Instruction> = vec![];
        while !self.eof() {
            let instr = self.parse_instruction()?;
            instrs.push(instr);
        }
        Ok(instrs)
    }
}

fn parse_code(source: &str) -> Vec<u8> {
    source
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let two_char = str::from_utf8(pair).unwrap();
            u8::from_str_radix(two_char, 16).unwrap()
        })
        .collect()
}

fn decode_and_report(code: Vec<u8>) {
    let mut decoder = InstDecoder::new(code);
    match decoder.decode() {
        Ok(d_instrs) => for instr in d_instrs {
            println!("{}", instr);
        },
        Err(e) => println!("Decode error: {:?} at {:X}", e, decoder.pos),
    }
}

pub fn problem_4_1() {
    use self::Instruction::*;
    let instrs = vec![
        IRMOVL(15, EBX),
        RRMOVL(EBX, ECX),
        RMMOVL(ECX, -3, EBX),
        OPL(ADD, EBX, ECX),
        JMP(None, 0xc),
    ];
    let code = encode_instructions(instrs);
    for byte in code.iter() {
        print!("{:02X}", byte);
    }
    println!("");
    decode_and_report(code);
}

pub fn problem_4_2() {
    let code1 = "30f3fcffffff40630008000000";
    let code2 = "a06f80080200000030f30a00000090";
    let code3 = "50540700000010f0b01f";
    let code4 = "6113730004000000";
    let code5 = "6362a0f0";

    decode_and_report(parse_code(code1));
    println!("");
    decode_and_report(parse_code(code2));
    println!("");
    decode_and_report(parse_code(code3));
    println!("");
    decode_and_report(parse_code(code4));
    println!("");
    decode_and_report(parse_code(code5));
}

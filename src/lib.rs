use pyo3::{prelude::*, types::PyTuple};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]


// 定义数据处理对象
struct Register {a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, h: u8, l: u8}

#[pyclass]
struct SoC {reg: Register, pc: u16, sp: u16, cyc: u128, ime: bool, ram: [u8; 65536], rom: Vec<u8>}

// 全部的实例化方法
#[pymethods]
impl SoC {
    // 初始化
    #[new]
    fn new(_py: Python<'_>, rom_data_pytuple: &PyTuple) -> PyResult<Self> {
        let mut rom_data: Vec<u8> = rom_data_pytuple
            .iter()
            .map(|item| item.extract::<u8>())
            .collect::<PyResult<_>>()?;
        rom_data.extend(&[0, 0]);
        Ok(Self {
            reg: Register { a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0 },
            pc: 0,
            sp: 0,
            cyc: 0,
            ime: false,
            ram: [0; 65536],
            rom: rom_data,
        })
    }
    // 取寄存器 r8
    fn get_r8(&self, r8pos: u8) -> u8 {
        match r8pos {
            0 => self.reg.a,
            1 => self.reg.f,
            2 => self.reg.b,
            3 => self.reg.c,
            4 => self.reg.d,
            5 => self.reg.e,
            6 => self.reg.h,
            7 => self.reg.l,
            _ => panic!("非法的寄存器位置：{}", r8pos)
        }
    }
    // 写寄存器 r8
    fn set_r8(&mut self, r8pos: u8, new_r8: u8) {
        match r8pos {
            0 => self.reg.a = new_r8,
            1 => self.reg.f = new_r8,
            2 => self.reg.b = new_r8,
            3 => self.reg.c = new_r8,
            4 => self.reg.d = new_r8,
            5 => self.reg.e = new_r8,
            6 => self.reg.h = new_r8,
            7 => self.reg.l = new_r8,
            _ => panic!("非法的寄存器位置: {}", r8pos),
        }
    }
    // 取寄存器 r16
    fn get_r16(&self, r16pos: (u8, u8)) -> u16 {
        match r16pos {
            (0, 1) => ((self.reg.a as u16) << 8) + (self.reg.f as u16),
            (2, 3) => ((self.reg.b as u16) << 8) + (self.reg.c as u16),
            (4, 5) => ((self.reg.d as u16) << 8) + (self.reg.e as u16),
            (6, 7) => ((self.reg.h as u16) << 8) + (self.reg.l as u16),
            _ => panic!("非法的寄存器位置: {}, {}", r16pos.0, r16pos.1),
        }
    }
    // 写寄存器 r16
    fn set_r16(&mut self, r16pos: (u8, u8), new_r16: u16) {
        match r16pos {
            (0, 1) => {self.set_r8(0, (new_r16 >> 8)  as u8); self.set_r8(1, (new_r16 & 0xff) as u8)},
            (2, 3) => {self.set_r8(2, (new_r16 >> 8)  as u8); self.set_r8(3, (new_r16 & 0xff) as u8)},
            (4, 5) => {self.set_r8(4, (new_r16 >> 8)  as u8); self.set_r8(5, (new_r16 & 0xff) as u8)},
            (6, 7) => {self.set_r8(6, (new_r16 >> 8)  as u8); self.set_r8(7, (new_r16 & 0xff) as u8)},
            _ => panic!("非法的寄存器位置: {}, {}", r16pos.0, r16pos.1),
        }
    }
    // 自增 r8
    fn r8_inc(&mut self, r8pos: u8) {
        match r8pos {
            0 => self.reg.a += 1,
            1 => self.reg.f += 1,
            2 => self.reg.b += 1,
            3 => self.reg.c += 1,
            4 => self.reg.d += 1,
            5 => self.reg.e += 1,
            6 => self.reg.h += 1,
            7 => self.reg.l += 1,
            _ => panic!("非法的寄存器位置: {}", r8pos),
        }
    }
    // 自减 r8
    fn r8_dec(&mut self, r8pos: u8) {
        match r8pos {
            0 => self.reg.a -= 1,
            1 => self.reg.f -= 1,
            2 => self.reg.b -= 1,
            3 => self.reg.c -= 1,
            4 => self.reg.d -= 1,
            5 => self.reg.e -= 1,
            6 => self.reg.h -= 1,
            7 => self.reg.l -= 1,
            _ => panic!("非法的寄存器位置: {}", r8pos),
        }
    }
    // 自增 r16
    fn r16_inc(&mut self, r16pos: (u8, u8)) {
        match r16pos {
               (0, 1) => {
                let new_r16 = self.get_r16(r16pos) + 1;
                self.set_r8(0, (new_r16 >> 8)  as u8);
                self.set_r8(1, (new_r16 & 0xff) as u8)
            }, (2, 3) => {
                let new_r16 = self.get_r16(r16pos) + 1;
                self.set_r8(2, (new_r16 >> 8)  as u8);
                self.set_r8(3, (new_r16 & 0xff) as u8)
            }, (4, 5) => {
                let new_r16 = self.get_r16(r16pos) + 1;
                self.set_r8(4, (new_r16 >> 8)  as u8);
                self.set_r8(5, (new_r16 & 0xff) as u8)
            }, (6, 7) => {
                let new_r16 = self.get_r16(r16pos) + 1;
                self.set_r8(6, (new_r16 >> 8)  as u8);
                self.set_r8(7, (new_r16 & 0xff) as u8)
            }, _ => panic!("非法的寄存器位置: {}, {}", r16pos.0, r16pos.1),
        }
    }
    // 自减 r16
    fn r16_dec(&mut self, r16pos: (u8, u8)) {
        match r16pos {
               (0, 1) => {
                let new_r16 = self.get_r16(r16pos) - 1;
                self.set_r8(0, (new_r16 >> 8)  as u8);
                self.set_r8(1, (new_r16 & 0xff) as u8)
            }, (2, 3) => {
                let new_r16 = self.get_r16(r16pos) - 1;
                self.set_r8(2, (new_r16 >> 8)  as u8);
                self.set_r8(3, (new_r16 & 0xff) as u8)
            }, (4, 5) => {
                let new_r16 = self.get_r16(r16pos) - 1;
                self.set_r8(4, (new_r16 >> 8)  as u8);
                self.set_r8(5, (new_r16 & 0xff) as u8)
            }, (6, 7) => {
                let new_r16 = self.get_r16(r16pos) - 1;
                self.set_r8(6, (new_r16 >> 8)  as u8);
                self.set_r8(7, (new_r16 & 0xff) as u8)
            }, _ => panic!("非法的寄存器位置: {}, {}", r16pos.0, r16pos.1),
        }
    }
    // 设置 bit
    fn r8_set(&mut self, r8pos: u8, b3: u8) {
        match r8pos {
            0 => self.reg.a = self.reg.a | (1 << b3),
            1 => self.reg.f = self.reg.f | (1 << b3),
            2 => self.reg.b = self.reg.b | (1 << b3),
            3 => self.reg.c = self.reg.c | (1 << b3),
            4 => self.reg.d = self.reg.d | (1 << b3),
            5 => self.reg.e = self.reg.e | (1 << b3),
            6 => self.reg.h = self.reg.h | (1 << b3),
            7 => self.reg.l = self.reg.l | (1 << b3),
            _ => panic!("非法的寄存器位置: {}", r8pos),
        }
    }
    // 清除 bit
    fn r8_res(&mut self, r8pos: u8, b3: u8) {
        match r8pos {
            0 => self.reg.a = self.reg.a & (0xff - (1 << b3)),
            1 => self.reg.f = self.reg.f & (0xff - (1 << b3)),
            2 => self.reg.b = self.reg.b & (0xff - (1 << b3)),
            3 => self.reg.c = self.reg.c & (0xff - (1 << b3)),
            4 => self.reg.d = self.reg.d & (0xff - (1 << b3)),
            5 => self.reg.e = self.reg.e & (0xff - (1 << b3)),
            6 => self.reg.h = self.reg.h & (0xff - (1 << b3)),
            7 => self.reg.l = self.reg.l & (0xff - (1 << b3)),
            _ => panic!("非法的寄存器位置: {}", r8pos),
        }
    }
    // 取 flag
    fn get_flag(&self, flag_bit: u8) -> u8 {
        match flag_bit {
            4 => (self.reg.f >> 4) & 1,
            5 => (self.reg.f >> 5) & 1,
            6 => (self.reg.f >> 6) & 1,
            7 => (self.reg.f >> 7) & 1,
            _ => panic!("非法的 flag 位: {}", flag_bit),
        }
    }
    // 设 flag
    fn set_flag(&mut self, flag_bit: u8) {
        match flag_bit {
            4 => self.reg.f = self.reg.f | (1 << flag_bit),
            5 => self.reg.f = self.reg.f | (1 << flag_bit),
            6 => self.reg.f = self.reg.f | (1 << flag_bit),
            7 => self.reg.f = self.reg.f | (1 << flag_bit),
            _ => panic!("非法的 flag 位: {}", flag_bit),
        }
    }
    // 清 flag
    fn res_flag(&mut self, flag_bit: u8) {
        match flag_bit {
            4 => self.reg.f = self.reg.f & (0xff - (1 << flag_bit)),
            5 => self.reg.f = self.reg.f & (0xff - (1 << flag_bit)),
            6 => self.reg.f = self.reg.f & (0xff - (1 << flag_bit)),
            7 => self.reg.f = self.reg.f & (0xff - (1 << flag_bit)),
            _ => panic!("非法的 flag 位: {}", flag_bit),
        }
    }
    // 取 SP
    fn get_sp(&self) -> u16 {
        self.sp
    }
    // 设 SP
    fn set_sp(&mut self, new_sp: u16) {
        self.sp = new_sp
    }
    // 取 PC
    fn get_pc(&self) -> u16 {
        self.pc
    }
    // 设 PC
    fn set_pc(&mut self, new_pc: u16) {
        self.pc = new_pc
    }
    // 增加 PC
    fn pc_inc(&mut self, n: u16) {
        self.pc += n
    }
    // 减少 PC
    fn pc_dec(&mut self, n: u16) {
        self.pc -= n
    }
    // 取 CYC
    fn get_cyc(&self) -> u128 {
        self.cyc
    }
    // 增加 CYC
    fn cyc_inc(&mut self, n: u128) {
        self.cyc += n
    }
    // 取 RAM
    fn ram_read(&self, addr: u16) -> u8 {
        if let Some(&data) = self.ram.get(addr as usize) {
            data
        } else {
            panic!("内存索引越界！合法范围：0-65535，您索引的地址：{}", addr)
        }
    }
    // 写 RAM
    fn ram_write(&mut self, addr: u16, data: u8) {
        if self.ram.get(addr as usize).is_some() {
            self.ram[addr as usize] = data;
        } else {
            panic!("内存索引越界！合法范围：0-65535，您索引的地址：{}", addr)
        }
    }
    // 读取全部 RAM
    fn ram_data(&self) -> [u8; 65536] {
        self.ram
    }
    // 取 ROM
    fn read_rom(&self, addr: u16) -> u8 {
        if let Some(&data) = self.rom.get(addr as usize) {
            data
        } else {
            panic!("ROM 索引越界！您索引的地址：{}", addr)
        }
    }
    // halt 信号发出
    fn halt(&self) -> bool {
        self.read_rom(self.get_pc()) == 0x76
    }
    // 启用 IME
    fn set_ime(&mut self) {
        self.ime = true
    }
    // 禁止 IME
    fn res_ime(&mut self) {
        self.ime = false
    }
    // 智能 flag
    fn smart_flag(&mut self, num1: u16, num2: u16, n: bool, u8_mode: bool) {
        match u8_mode {
            true => {
                let num1_lo4: u8 = (num1 & 0xf) as u8;
                let num2_lo4: u8 = (num2 & 0xf) as u8;

                if n == false {
                    if (num1 + num2) & 0xff == 0 { self.set_flag(7) } else { self.res_flag(7) };
                    self.res_flag(6);
                    if num1_lo4 + num2_lo4 > 0xf { self.set_flag(5) } else { self.res_flag(5) };
                    if num1+ num2 > 0xff { self.set_flag(4) } else { self.res_flag(4) };
                } else {
                    if (num1 as i16) - (num2 as i16) == 0 { self.set_flag(7) } else { self.res_flag(7) };
                    self.set_flag(6);
                    if (num1_lo4 as i8) - (num2_lo4 as i8) < 0 { self.set_flag(5) } else { self.res_flag(5) };
                    if (num1 as i16) - (num2 as i16) < 0 { self.set_flag(4) } else { self.res_flag(4) };
                }
            },
            false => {
                let num1: u32 = num1 as u32;
                let num2: u32 = num2 as u32;
                let num1_lo12: u32 = num1 & 0xfff;
                let num2_lo12: u32 = num2 & 0xfff;

                if n == false {
                    self.res_flag(6);
                    if num1_lo12 + num2_lo12 > 0xfff { self.set_flag(5) } else { self.res_flag(5) };
                    if num1 + num2 > 0xffff { self.set_flag(4) } else { self.res_flag(4) };
                } else {
                    self.set_flag(6);
                    if (num1_lo12 as i32) - (num2_lo12 as i32) < 0 { self.set_flag(5) } else { self.res_flag(5) };
                    if (num1 as i32) - (num2 as i32) < 0 { self.set_flag(4) } else { self.res_flag(4) };
                }
            }
        }
    }
    // 根据区间提供操作码
    fn give_opt_code(&self, length: u8) -> Vec<u8> {
        return_instruction(self, length)
    }

    // 查看寄存器状况
    fn disp(&self) {
        println!("+----------- GameBoy SM83 Register Display -----------+");
        println!("|Flag Register: {} - {} - {} - {}               IME: {}|", 
        if (self.get_flag(7)) == 1 {"Z"} else {"0"},
        if (self.get_flag(6)) == 1 {"N"} else {"0"},
        if (self.get_flag(5)) == 1 {"H"} else {"0"},
        if (self.get_flag(4)) == 1 {"C"} else {"0"},
        if (self.ime) == true {"TRUE "} else {"FALSE"});
        println!("|R16:        {:5},      {:5},      {:5},      {:5}|",
                 self.get_r16((0, 1)), self.get_r16((2, 3)), self.get_r16((4, 5)), self.get_r16((6, 7)));
        println!("|R8_10: {:4}, {:4}, {:4}, {:4}, {:4}, {:4}, {:4}, {:4}|", 
                 self.reg.a, self.reg.f, self.reg.b, self.reg.c,
                 self.reg.d, self.reg.e, self.reg.h, self.reg.l);
        println!("|R8_16: 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}|", 
                 self.reg.a, self.reg.f, self.reg.b, self.reg.c,
                 self.reg.d, self.reg.e, self.reg.h, self.reg.l);
        println!("|A: {0:08b},  F: {1:08b},  B: {2:08b},  C: {3:08b}|",
                 self.reg.a, self.reg.f, self.reg.b, self.reg.c);
        println!("|D: {0:08b},  E: {1:08b},  H: {2:08b},  L: {3:08b}|",
                 self.reg.d, self.reg.e, self.reg.h, self.reg.l);
        println!("|PC:{:08},  SP:0x{:04x},    INST_CYC:{:16}|",
                 self.pc, self.sp, self.cyc);
        println!("+----------ROM Next 3 Bytes [0x{0:02x} 0x{1:02x} 0x{2:02x}]----------+\n",
                 self.read_rom(self.get_pc()), self.read_rom(self.get_pc()+1), self.read_rom(self.get_pc()+2),)
    }
    // 显示寄存器 r8
    fn disp_r8(&self) {
        println!("============ GameBoy SM83 R8B Display ============");
        println!("Flag Register: {} - {} - {} - {}", 
                 if (self.get_flag(7)) == 1 {"Z"} else {"0"},
                 if (self.get_flag(6)) == 1 {"N"} else {"0"},
                 if (self.get_flag(5)) == 1 {"H"} else {"0"},
                 if (self.get_flag(4)) == 1 {"C"} else {"0"});
        println!("A: {0:08b}, F: {1:08b}, B: {2:08b}, C: {3:08b}", self.reg.a, self.reg.f, self.reg.b, self.reg.c);
        println!("D: {0:08b}, E: {1:08b}, H: {2:08b}, L: {3:08b}\n", self.reg.d, self.reg.e, self.reg.h, self.reg.l);
    }

    // 通过 ROM 直接执行一步命令
    fn one_step(&mut self) {process_by_step(self);}
}


// 调配函数
fn return_instruction(soc: &SoC, length: u8) -> Vec<u8> {
    let mut opt_code = Vec::new();
    let start_point = soc.get_pc();

    for i in start_point..(start_point + length as u16) {
        opt_code.push(soc.read_rom(i));
    }
    opt_code
}


fn process_by_step(soc: &mut SoC) {
    let code = [
        soc.read_rom(soc.get_pc()) as u16,
        soc.read_rom(soc.get_pc() + 1) as u16,
        soc.read_rom(soc.get_pc() + 2) as u16,
    ];
    match code[0] as u8 {
        0x00 => {nop(soc)},
        0x01 => {ld_r16_n16(soc, R16::BC, b8x2(code[1], code[2]))},
        0x02 => {ld_r16ram_a(soc, R16RAM::BC)},
        0x03 => {inc_r16(soc, R16::BC)},
        0x04 => {inc_r8(soc, R8::B)},
        0x05 => {dec_r8(soc, R8::B)},
        0x06 => {ld_r8_n8(soc, R8::B, code[1])},
        0x07 => {rlca(soc)},
        0x08 => {ld_a16_sp(soc, b8x2_le(code[1], code[2]))},
        0x09 => {add_hl_r16(soc, R16::BC)},
        0x0A => {ld_a_r16ram(soc, R16RAM::BC)},
        0x0B => {dec_r16(soc, R16::BC)},
        0x0C => {inc_r8(soc, R8::C)},
        0x0D => {dec_r8(soc, R8::C)},
        0x0E => {ld_r8_n8(soc, R8::C, code[1])},
        0x0F => {rrca(soc)},
        0x10 => {stop(soc)},
        0x11 => {ld_r16_n16(soc, R16::DE, b8x2(code[1], code[2]))},
        0x12 => {ld_r16ram_a(soc, R16RAM::DE)},
        0x13 => {inc_r16(soc, R16::DE)},
        0x14 => {inc_r8(soc, R8::D)},
        0x15 => {dec_r8(soc, R8::D)},
        0x16 => {ld_r8_n8(soc, R8::D, code[1])},
        0x17 => {rla(soc)},
        0x18 => {jr_e8(soc, code[1] as i8)},
        0x19 => {add_hl_r16(soc, R16::DE)},
        0x1A => {ld_a_r16ram(soc, R16RAM::DE)},
        0x1B => {dec_r16(soc, R16::DE)},
        0x1C => {inc_r8(soc, R8::E)},
        0x1D => {dec_r8(soc, R8::E)},
        0x1E => {ld_r8_n8(soc, R8::E, code[1])},
        0x1F => {rra(soc)},
        0x20 => {jr_cond_e8(soc, COND::NZ, code[1])},
        0x21 => {ld_r16_n16(soc, R16::HL, b8x2(code[1], code[2]))},
        0x22 => {ld_r16ram_a(soc, R16RAM::HLI)},
        0x23 => {inc_r16(soc, R16::HL)},
        0x24 => {inc_r8(soc, R8::H)},
        0x25 => {dec_r8(soc, R8::H)},
        0x26 => {ld_r8_n8(soc, R8::H, code[1])},
        0x27 => {daa(soc)},
        0x28 => {jr_cond_e8(soc, COND::Z, code[1])},
        0x29 => {add_hl_r16(soc, R16::HL)},
        0x2A => {ld_a_r16ram(soc, R16RAM::HLI)},
        0x2B => {dec_r16(soc, R16::HL)},
        0x2C => {inc_r8(soc, R8::L)},
        0x2D => {dec_r8(soc, R8::L)},
        0x2E => {ld_r8_n8(soc, R8::L, code[1])},
        0x2F => {cpl(soc)},
        0x30 => {jr_cond_e8(soc, COND::NC, code[1])},
        0x31 => {ld_r16_n16(soc, R16::SP, b8x2(code[1], code[2]))},
        0x32 => {ld_r16ram_a(soc, R16RAM::HLD)},
        0x33 => {inc_r16(soc, R16::SP)},
        0x34 => {inc_r8(soc, R8::HL)},
        0x35 => {dec_r8(soc, R8::HL)},
        0x36 => {ld_r8_n8(soc, R8::HL, code[1])},
        0x37 => {scf(soc)},
        0x38 => {jr_cond_e8(soc, COND::C, code[1])},
        0x39 => {add_hl_r16(soc, R16::SP)},
        0x3A => {ld_a_r16ram(soc, R16RAM::HLD)},
        0x3B => {dec_r16(soc, R16::SP)},
        0x3C => {inc_r8(soc, R8::A)},
        0x3D => {dec_r8(soc, R8::A)},
        0x3E => {ld_r8_n8(soc, R8::A, code[1])},
        0x3F => {ccf(soc)},
        0x40 => {ld_r8_r8(soc, R8::B, R8::B)},
        0x41 => {ld_r8_r8(soc, R8::B, R8::C)},
        0x42 => {ld_r8_r8(soc, R8::B, R8::D)},
        0x43 => {ld_r8_r8(soc, R8::B, R8::E)},
        0x44 => {ld_r8_r8(soc, R8::B, R8::H)},
        0x45 => {ld_r8_r8(soc, R8::B, R8::L)},
        0x46 => {ld_r8_r8(soc, R8::B, R8::HL)},
        0x47 => {ld_r8_r8(soc, R8::B, R8::A)},
        0x48 => {ld_r8_r8(soc, R8::C, R8::B)},
        0x49 => {ld_r8_r8(soc, R8::C, R8::C)},
        0x4A => {ld_r8_r8(soc, R8::C, R8::D)},
        0x4B => {ld_r8_r8(soc, R8::C, R8::E)},
        0x4C => {ld_r8_r8(soc, R8::C, R8::H)},
        0x4D => {ld_r8_r8(soc, R8::C, R8::L)},
        0x4E => {ld_r8_r8(soc, R8::C, R8::HL)},
        0x4F => {ld_r8_r8(soc, R8::C, R8::A)},
        0x50 => {ld_r8_r8(soc, R8::D, R8::B)},
        0x51 => {ld_r8_r8(soc, R8::D, R8::C)},
        0x52 => {ld_r8_r8(soc, R8::D, R8::D)},
        0x53 => {ld_r8_r8(soc, R8::D, R8::E)},
        0x54 => {ld_r8_r8(soc, R8::D, R8::H)},
        0x55 => {ld_r8_r8(soc, R8::D, R8::L)},
        0x56 => {ld_r8_r8(soc, R8::D, R8::HL)},
        0x57 => {ld_r8_r8(soc, R8::D, R8::A)},
        0x58 => {ld_r8_r8(soc, R8::E, R8::B)},
        0x59 => {ld_r8_r8(soc, R8::E, R8::C)},
        0x5A => {ld_r8_r8(soc, R8::E, R8::D)},
        0x5B => {ld_r8_r8(soc, R8::E, R8::E)},
        0x5C => {ld_r8_r8(soc, R8::E, R8::H)},
        0x5D => {ld_r8_r8(soc, R8::E, R8::L)},
        0x5E => {ld_r8_r8(soc, R8::E, R8::HL)},
        0x5F => {ld_r8_r8(soc, R8::E, R8::A)},
        0x60 => {ld_r8_r8(soc, R8::H, R8::B)},
        0x61 => {ld_r8_r8(soc, R8::H, R8::C)},
        0x62 => {ld_r8_r8(soc, R8::H, R8::D)},
        0x63 => {ld_r8_r8(soc, R8::H, R8::E)},
        0x64 => {ld_r8_r8(soc, R8::H, R8::H)},
        0x65 => {ld_r8_r8(soc, R8::H, R8::L)},
        0x66 => {ld_r8_r8(soc, R8::H, R8::HL)},
        0x67 => {ld_r8_r8(soc, R8::H, R8::A)},
        0x68 => {ld_r8_r8(soc, R8::L, R8::B)},
        0x69 => {ld_r8_r8(soc, R8::L, R8::C)},
        0x6A => {ld_r8_r8(soc, R8::L, R8::D)},
        0x6B => {ld_r8_r8(soc, R8::L, R8::E)},
        0x6C => {ld_r8_r8(soc, R8::L, R8::H)},
        0x6D => {ld_r8_r8(soc, R8::L, R8::L)},
        0x6E => {ld_r8_r8(soc, R8::L, R8::HL)},
        0x6F => {ld_r8_r8(soc, R8::L, R8::A)},
        0x70 => {ld_r8_r8(soc, R8::HL, R8::B)},
        0x71 => {ld_r8_r8(soc, R8::HL, R8::C)},
        0x72 => {ld_r8_r8(soc, R8::HL, R8::D)},
        0x73 => {ld_r8_r8(soc, R8::HL, R8::E)},
        0x74 => {ld_r8_r8(soc, R8::HL, R8::H)},
        0x75 => {ld_r8_r8(soc, R8::HL, R8::L)},
        0x76 => {halt(soc);},
        0x77 => {ld_r8_r8(soc, R8::HL, R8::A)},
        0x78 => {ld_r8_r8(soc, R8::A, R8::B)},
        0x79 => {ld_r8_r8(soc, R8::A, R8::C)},
        0x7A => {ld_r8_r8(soc, R8::A, R8::D)},
        0x7B => {ld_r8_r8(soc, R8::A, R8::E)},
        0x7C => {ld_r8_r8(soc, R8::A, R8::H)},
        0x7D => {ld_r8_r8(soc, R8::A, R8::L)},
        0x7E => {ld_r8_r8(soc, R8::A, R8::HL)},
        0x7F => {ld_r8_r8(soc, R8::A, R8::A)},
        0x80 => {alu_a_r8(soc, R8::B,  ALU3::ADD)},
        0x81 => {alu_a_r8(soc, R8::C,  ALU3::ADD)},
        0x82 => {alu_a_r8(soc, R8::D,  ALU3::ADD)},
        0x83 => {alu_a_r8(soc, R8::E,  ALU3::ADD)},
        0x84 => {alu_a_r8(soc, R8::H,  ALU3::ADD)},
        0x85 => {alu_a_r8(soc, R8::L,  ALU3::ADD)},
        0x86 => {alu_a_r8(soc, R8::HL, ALU3::ADD)},
        0x87 => {alu_a_r8(soc, R8::A,  ALU3::ADD)},
        0x88 => {alu_a_r8(soc, R8::B,  ALU3::ADC)},
        0x89 => {alu_a_r8(soc, R8::C,  ALU3::ADC)},
        0x8A => {alu_a_r8(soc, R8::D,  ALU3::ADC)},
        0x8B => {alu_a_r8(soc, R8::E,  ALU3::ADC)},
        0x8C => {alu_a_r8(soc, R8::H,  ALU3::ADC)},
        0x8D => {alu_a_r8(soc, R8::L,  ALU3::ADC)},
        0x8E => {alu_a_r8(soc, R8::HL, ALU3::ADC)},
        0x8F => {alu_a_r8(soc, R8::A,  ALU3::ADC)},
        0x90 => {alu_a_r8(soc, R8::B,  ALU3::SUB)},
        0x91 => {alu_a_r8(soc, R8::C,  ALU3::SUB)},
        0x92 => {alu_a_r8(soc, R8::D,  ALU3::SUB)},
        0x93 => {alu_a_r8(soc, R8::E,  ALU3::SUB)},
        0x94 => {alu_a_r8(soc, R8::H,  ALU3::SUB)},
        0x95 => {alu_a_r8(soc, R8::L,  ALU3::SUB)},
        0x96 => {alu_a_r8(soc, R8::HL, ALU3::SUB)},
        0x97 => {alu_a_r8(soc, R8::A,  ALU3::SUB)},
        0x98 => {alu_a_r8(soc, R8::B,  ALU3::SBC)},
        0x99 => {alu_a_r8(soc, R8::C,  ALU3::SBC)},
        0x9A => {alu_a_r8(soc, R8::D,  ALU3::SBC)},
        0x9B => {alu_a_r8(soc, R8::E,  ALU3::SBC)},
        0x9C => {alu_a_r8(soc, R8::H,  ALU3::SBC)},
        0x9D => {alu_a_r8(soc, R8::L,  ALU3::SBC)},
        0x9E => {alu_a_r8(soc, R8::HL, ALU3::SBC)},
        0x9F => {alu_a_r8(soc, R8::A,  ALU3::SBC)},
        0xA0 => {alu_a_r8(soc, R8::B,  ALU3::AND)},
        0xA1 => {alu_a_r8(soc, R8::C,  ALU3::AND)},
        0xA2 => {alu_a_r8(soc, R8::D,  ALU3::AND)},
        0xA3 => {alu_a_r8(soc, R8::E,  ALU3::AND)},
        0xA4 => {alu_a_r8(soc, R8::H,  ALU3::AND)},
        0xA5 => {alu_a_r8(soc, R8::L,  ALU3::AND)},
        0xA6 => {alu_a_r8(soc, R8::HL, ALU3::AND)},
        0xA7 => {alu_a_r8(soc, R8::A,  ALU3::AND)},
        0xA8 => {alu_a_r8(soc, R8::B,  ALU3::XOR)},
        0xA9 => {alu_a_r8(soc, R8::C,  ALU3::XOR)},
        0xAA => {alu_a_r8(soc, R8::D,  ALU3::XOR)},
        0xAB => {alu_a_r8(soc, R8::E,  ALU3::XOR)},
        0xAC => {alu_a_r8(soc, R8::H,  ALU3::XOR)},
        0xAD => {alu_a_r8(soc, R8::L,  ALU3::XOR)},
        0xAE => {alu_a_r8(soc, R8::HL, ALU3::XOR)},
        0xAF => {alu_a_r8(soc, R8::A,  ALU3::XOR)},
        0xB0 => {alu_a_r8(soc, R8::B,  ALU3::OR )},
        0xB1 => {alu_a_r8(soc, R8::C,  ALU3::OR )},
        0xB2 => {alu_a_r8(soc, R8::D,  ALU3::OR )},
        0xB3 => {alu_a_r8(soc, R8::E,  ALU3::OR )},
        0xB4 => {alu_a_r8(soc, R8::H,  ALU3::OR )},
        0xB5 => {alu_a_r8(soc, R8::L,  ALU3::OR )},
        0xB6 => {alu_a_r8(soc, R8::HL, ALU3::OR )},
        0xB7 => {alu_a_r8(soc, R8::A,  ALU3::OR )},
        0xB8 => {alu_a_r8(soc, R8::B,  ALU3::CP )},
        0xB9 => {alu_a_r8(soc, R8::C,  ALU3::CP )},
        0xBA => {alu_a_r8(soc, R8::D,  ALU3::CP )},
        0xBB => {alu_a_r8(soc, R8::E,  ALU3::CP )},
        0xBC => {alu_a_r8(soc, R8::H,  ALU3::CP )},
        0xBD => {alu_a_r8(soc, R8::L,  ALU3::CP )},
        0xBE => {alu_a_r8(soc, R8::HL, ALU3::CP )},
        0xBF => {alu_a_r8(soc, R8::A,  ALU3::CP )},
        0xC0 => {ret_cond(soc, COND::NZ)},
        0xC1 => {pop_r16stk(soc, R16STK::BC)},
        0xC2 => {jp_cond_a16(soc, COND::NZ, b8x2_le(code[1], code[2]))},
        0xC3 => {jp_a16(soc, b8x2_le(code[1], code[2]))},
        0xC4 => {call_cond_a16(soc, COND::NZ, b8x2_le(code[1], code[2]))},
        0xC5 => {push_r16stk(soc, R16STK::BC)},
        0xC6 => {alu_a_n8(soc, code[1], ALU3::ADC)},
        0xC7 => {rst_tgt3(soc, TGT3::T0)},
        0xC8 => {ret_cond(soc, COND::Z)},
        0xC9 => {ret(soc)},
        0xCA => {jp_cond_a16(soc, COND::Z, b8x2_le(code[1], code[2]))},
        0xCB => {ex_inst(soc, code[1]);},
        0xCC => {call_cond_a16(soc, COND::Z, b8x2_le(code[1], code[2]))},
        0xCD => {call_a16(soc, b8x2_le(code[1], code[2]))},
        0xCE => {alu_a_n8(soc, code[1], ALU3::ADD)},
        0xCF => {rst_tgt3(soc, TGT3::T1)},
        0xD0 => {ret_cond(soc, COND::NC)},
        0xD1 => {pop_r16stk(soc, R16STK::DE)},
        0xD2 => {jp_cond_a16(soc, COND::NC, b8x2_le(code[1], code[2]))},
        0xD3 => {},
        0xD4 => {call_cond_a16(soc, COND::NC, b8x2_le(code[1], code[2]))},
        0xD5 => {push_r16stk(soc, R16STK::DE)},
        0xD6 => {alu_a_n8(soc, code[1], ALU3::SUB)},
        0xD7 => {rst_tgt3(soc, TGT3::T2)},
        0xD8 => {ret_cond(soc, COND::C)},
        0xD9 => {reti(soc)},
        0xDA => {jp_cond_a16(soc, COND::C, b8x2_le(code[1], code[2]))},
        0xDB => {},
        0xDC => {call_cond_a16(soc, COND::C, b8x2_le(code[1], code[2]))},
        0xDD => {},
        0xDE => {alu_a_n8(soc, code[1], ALU3::SBC)},
        0xDF => {rst_tgt3(soc, TGT3::T3)},
        0xE0 => {ldh_a8_a(soc, code[1] as u8)},
        0xE1 => {pop_r16stk(soc, R16STK::HL)},
        0xE2 => {ldh_c_a(soc)},
        0xE3 => {},
        0xE4 => {},
        0xE5 => {push_r16stk(soc, R16STK::HL)},
        0xE6 => {alu_a_n8(soc, code[1], ALU3::AND)},
        0xE7 => {rst_tgt3(soc, TGT3::T4)},
        0xE8 => {add_sp_e8(soc, code[1] as i8)},
        0xE9 => {jp_hl(soc)},
        0xEA => {ld_a16_a(soc, b8x2_le(code[1], code[2]))},
        0xEB => {},
        0xEC => {},
        0xED => {},
        0xEE => {alu_a_n8(soc, code[1], ALU3::XOR)},
        0xEF => {rst_tgt3(soc, TGT3::T5)},
        0xF0 => {ldh_a_a8(soc, code[1] as u8)},
        0xF1 => {pop_r16stk(soc, R16STK::AF)},
        0xF2 => {ldh_a_c(soc)},
        0xF3 => {di(soc)},
        0xF4 => {},
        0xF5 => {push_r16stk(soc, R16STK::AF)},
        0xF6 => {alu_a_n8(soc, code[1], ALU3::OR)},
        0xF7 => {rst_tgt3(soc, TGT3::T6)},
        0xF8 => {ld_hl_sp_e8(soc, code[1] as i8)},
        0xF9 => {ld_sp_hl(soc)},
        0xFA => {ld_a_a16(soc, b8x2_le(code[1], code[2]))},
        0xFB => {ei(soc)},
        0xFC => {},
        0xFD => {},
        0xFE => {alu_a_n8(soc, code[1], ALU3::CP)},
        0xFF => {rst_tgt3(soc, TGT3::T7)},
    }
}

fn ex_inst(soc: &mut SoC, sub_code: u16) {
    let sub_code = sub_code as u8;
    match sub_code {
        0x00 => {bop_r8(soc, R8::B,  BOP3::RLC );}
        0x01 => {bop_r8(soc, R8::C,  BOP3::RLC );}
        0x02 => {bop_r8(soc, R8::D,  BOP3::RLC );}
        0x03 => {bop_r8(soc, R8::E,  BOP3::RLC );}
        0x04 => {bop_r8(soc, R8::H,  BOP3::RLC );}
        0x05 => {bop_r8(soc, R8::L,  BOP3::RLC );}
        0x06 => {bop_r8(soc, R8::HL, BOP3::RLC );}
        0x07 => {bop_r8(soc, R8::A,  BOP3::RLC );}
        0x08 => {bop_r8(soc, R8::B,  BOP3::RRC );}
        0x09 => {bop_r8(soc, R8::C,  BOP3::RRC );}
        0x0A => {bop_r8(soc, R8::D,  BOP3::RRC );}
        0x0B => {bop_r8(soc, R8::E,  BOP3::RRC );}
        0x0C => {bop_r8(soc, R8::H,  BOP3::RRC );}
        0x0D => {bop_r8(soc, R8::L,  BOP3::RRC );}
        0x0E => {bop_r8(soc, R8::HL, BOP3::RRC );}
        0x0F => {bop_r8(soc, R8::A,  BOP3::RRC );}
        0x10 => {bop_r8(soc, R8::B,  BOP3::RL  );}
        0x11 => {bop_r8(soc, R8::C,  BOP3::RL  );}
        0x12 => {bop_r8(soc, R8::D,  BOP3::RL  );}
        0x13 => {bop_r8(soc, R8::E,  BOP3::RL  );}
        0x14 => {bop_r8(soc, R8::H,  BOP3::RL  );}
        0x15 => {bop_r8(soc, R8::L,  BOP3::RL  );}
        0x16 => {bop_r8(soc, R8::HL, BOP3::RL  );}
        0x17 => {bop_r8(soc, R8::A,  BOP3::RL  );}
        0x18 => {bop_r8(soc, R8::B,  BOP3::RR  );}
        0x19 => {bop_r8(soc, R8::C,  BOP3::RR  );}
        0x1A => {bop_r8(soc, R8::D,  BOP3::RR  );}
        0x1B => {bop_r8(soc, R8::E,  BOP3::RR  );}
        0x1C => {bop_r8(soc, R8::H,  BOP3::RR  );}
        0x1D => {bop_r8(soc, R8::L,  BOP3::RR  );}
        0x1E => {bop_r8(soc, R8::HL, BOP3::RR  );}
        0x1F => {bop_r8(soc, R8::A,  BOP3::RR  );}
        0x20 => {bop_r8(soc, R8::B,  BOP3::SLA );}
        0x21 => {bop_r8(soc, R8::C,  BOP3::SLA );}
        0x22 => {bop_r8(soc, R8::D,  BOP3::SLA );}
        0x23 => {bop_r8(soc, R8::E,  BOP3::SLA );}
        0x24 => {bop_r8(soc, R8::H,  BOP3::SLA );}
        0x25 => {bop_r8(soc, R8::L,  BOP3::SLA );}
        0x26 => {bop_r8(soc, R8::HL, BOP3::SLA );}
        0x27 => {bop_r8(soc, R8::A,  BOP3::SLA );}
        0x28 => {bop_r8(soc, R8::B,  BOP3::SRA );}
        0x29 => {bop_r8(soc, R8::C,  BOP3::SRA );}
        0x2A => {bop_r8(soc, R8::D,  BOP3::SRA );}
        0x2B => {bop_r8(soc, R8::E,  BOP3::SRA );}
        0x2C => {bop_r8(soc, R8::H,  BOP3::SRA );}
        0x2D => {bop_r8(soc, R8::L,  BOP3::SRA );}
        0x2E => {bop_r8(soc, R8::HL, BOP3::SRA );}
        0x2F => {bop_r8(soc, R8::A,  BOP3::SRA );}
        0x30 => {bop_r8(soc, R8::B,  BOP3::SWAP);}
        0x31 => {bop_r8(soc, R8::C,  BOP3::SWAP);}
        0x32 => {bop_r8(soc, R8::D,  BOP3::SWAP);}
        0x33 => {bop_r8(soc, R8::E,  BOP3::SWAP);}
        0x34 => {bop_r8(soc, R8::H,  BOP3::SWAP);}
        0x35 => {bop_r8(soc, R8::L,  BOP3::SWAP);}
        0x36 => {bop_r8(soc, R8::HL, BOP3::SWAP);}
        0x37 => {bop_r8(soc, R8::A,  BOP3::SWAP);}
        0x38 => {bop_r8(soc, R8::B,  BOP3::SRL );}
        0x39 => {bop_r8(soc, R8::C,  BOP3::SRL );}
        0x3A => {bop_r8(soc, R8::D,  BOP3::SRL );}
        0x3B => {bop_r8(soc, R8::E,  BOP3::SRL );}
        0x3C => {bop_r8(soc, R8::H,  BOP3::SRL );}
        0x3D => {bop_r8(soc, R8::L,  BOP3::SRL );}
        0x3E => {bop_r8(soc, R8::HL, BOP3::SRL );}
        0x3F => {bop_r8(soc, R8::A,  BOP3::SRL );}
        0x40 => {bit_r8_b3(soc, R8::B,  0);}
        0x41 => {bit_r8_b3(soc, R8::C,  0);}
        0x42 => {bit_r8_b3(soc, R8::D,  0);}
        0x43 => {bit_r8_b3(soc, R8::E,  0);}
        0x44 => {bit_r8_b3(soc, R8::H,  0);}
        0x45 => {bit_r8_b3(soc, R8::L,  0);}
        0x46 => {bit_r8_b3(soc, R8::HL, 0);}
        0x47 => {bit_r8_b3(soc, R8::A,  0);}
        0x48 => {bit_r8_b3(soc, R8::B,  1);}
        0x49 => {bit_r8_b3(soc, R8::C,  1);}
        0x4A => {bit_r8_b3(soc, R8::D,  1);}
        0x4B => {bit_r8_b3(soc, R8::E,  1);}
        0x4C => {bit_r8_b3(soc, R8::H,  1);}
        0x4D => {bit_r8_b3(soc, R8::L,  1);}
        0x4E => {bit_r8_b3(soc, R8::HL, 1);}
        0x4F => {bit_r8_b3(soc, R8::A,  1);}
        0x50 => {bit_r8_b3(soc, R8::B,  2);}
        0x51 => {bit_r8_b3(soc, R8::C,  2);}
        0x52 => {bit_r8_b3(soc, R8::D,  2);}
        0x53 => {bit_r8_b3(soc, R8::E,  2);}
        0x54 => {bit_r8_b3(soc, R8::H,  2);}
        0x55 => {bit_r8_b3(soc, R8::L,  2);}
        0x56 => {bit_r8_b3(soc, R8::HL, 2);}
        0x57 => {bit_r8_b3(soc, R8::A,  2);}
        0x58 => {bit_r8_b3(soc, R8::B,  3);}
        0x59 => {bit_r8_b3(soc, R8::C,  3);}
        0x5A => {bit_r8_b3(soc, R8::D,  3);}
        0x5B => {bit_r8_b3(soc, R8::E,  3);}
        0x5C => {bit_r8_b3(soc, R8::H,  3);}
        0x5D => {bit_r8_b3(soc, R8::L,  3);}
        0x5E => {bit_r8_b3(soc, R8::HL, 3);}
        0x5F => {bit_r8_b3(soc, R8::A,  3);}
        0x60 => {bit_r8_b3(soc, R8::B,  4);}
        0x61 => {bit_r8_b3(soc, R8::C,  4);}
        0x62 => {bit_r8_b3(soc, R8::D,  4);}
        0x63 => {bit_r8_b3(soc, R8::E,  4);}
        0x64 => {bit_r8_b3(soc, R8::H,  4);}
        0x65 => {bit_r8_b3(soc, R8::L,  4);}
        0x66 => {bit_r8_b3(soc, R8::HL, 4);}
        0x67 => {bit_r8_b3(soc, R8::A,  4);}
        0x68 => {bit_r8_b3(soc, R8::B,  5);}
        0x69 => {bit_r8_b3(soc, R8::C,  5);}
        0x6A => {bit_r8_b3(soc, R8::D,  5);}
        0x6B => {bit_r8_b3(soc, R8::E,  5);}
        0x6C => {bit_r8_b3(soc, R8::H,  5);}
        0x6D => {bit_r8_b3(soc, R8::L,  5);}
        0x6E => {bit_r8_b3(soc, R8::HL, 5);}
        0x6F => {bit_r8_b3(soc, R8::A,  5);}
        0x70 => {bit_r8_b3(soc, R8::B,  6);}
        0x71 => {bit_r8_b3(soc, R8::C,  6);}
        0x72 => {bit_r8_b3(soc, R8::D,  6);}
        0x73 => {bit_r8_b3(soc, R8::E,  6);}
        0x74 => {bit_r8_b3(soc, R8::H,  6);}
        0x75 => {bit_r8_b3(soc, R8::L,  6);}
        0x76 => {bit_r8_b3(soc, R8::HL, 6);}
        0x77 => {bit_r8_b3(soc, R8::A,  6);}
        0x78 => {bit_r8_b3(soc, R8::B,  7);}
        0x79 => {bit_r8_b3(soc, R8::C,  7);}
        0x7A => {bit_r8_b3(soc, R8::D,  7);}
        0x7B => {bit_r8_b3(soc, R8::E,  7);}
        0x7C => {bit_r8_b3(soc, R8::H,  7);}
        0x7D => {bit_r8_b3(soc, R8::L,  7);}
        0x7E => {bit_r8_b3(soc, R8::HL, 7);}
        0x7F => {bit_r8_b3(soc, R8::A,  7);}
        0x80 => {bit_r8_b3(soc, R8::B,  0);}
        0x81 => {bit_r8_b3(soc, R8::C,  0);}
        0x82 => {bit_r8_b3(soc, R8::D,  0);}
        0x83 => {bit_r8_b3(soc, R8::E,  0);}
        0x84 => {bit_r8_b3(soc, R8::H,  0);}
        0x85 => {bit_r8_b3(soc, R8::L,  0);}
        0x86 => {bit_r8_b3(soc, R8::HL, 0);}
        0x87 => {bit_r8_b3(soc, R8::A,  0);}
        0x88 => {bit_r8_b3(soc, R8::B,  1);}
        0x89 => {bit_r8_b3(soc, R8::C,  1);}
        0x8A => {bit_r8_b3(soc, R8::D,  1);}
        0x8B => {bit_r8_b3(soc, R8::E,  1);}
        0x8C => {bit_r8_b3(soc, R8::H,  1);}
        0x8D => {bit_r8_b3(soc, R8::L,  1);}
        0x8E => {bit_r8_b3(soc, R8::HL, 1);}
        0x8F => {bit_r8_b3(soc, R8::A,  1);}
        0x90 => {res_r8_b3(soc, R8::B,  2);}
        0x91 => {res_r8_b3(soc, R8::C,  2);}
        0x92 => {res_r8_b3(soc, R8::D,  2);}
        0x93 => {res_r8_b3(soc, R8::E,  2);}
        0x94 => {res_r8_b3(soc, R8::H,  2);}
        0x95 => {res_r8_b3(soc, R8::L,  2);}
        0x96 => {res_r8_b3(soc, R8::HL, 2);}
        0x97 => {res_r8_b3(soc, R8::A,  2);}
        0x98 => {res_r8_b3(soc, R8::B,  3);}
        0x99 => {res_r8_b3(soc, R8::C,  3);}
        0x9A => {res_r8_b3(soc, R8::D,  3);}
        0x9B => {res_r8_b3(soc, R8::E,  3);}
        0x9C => {res_r8_b3(soc, R8::H,  3);}
        0x9D => {res_r8_b3(soc, R8::L,  3);}
        0x9E => {res_r8_b3(soc, R8::HL, 3);}
        0x9F => {res_r8_b3(soc, R8::A,  3);}
        0xA0 => {res_r8_b3(soc, R8::B,  4);}
        0xA1 => {res_r8_b3(soc, R8::C,  4);}
        0xA2 => {res_r8_b3(soc, R8::D,  4);}
        0xA3 => {res_r8_b3(soc, R8::E,  4);}
        0xA4 => {res_r8_b3(soc, R8::H,  4);}
        0xA5 => {res_r8_b3(soc, R8::L,  4);}
        0xA6 => {res_r8_b3(soc, R8::HL, 4);}
        0xA7 => {res_r8_b3(soc, R8::A,  4);}
        0xA8 => {res_r8_b3(soc, R8::B,  5);}
        0xA9 => {res_r8_b3(soc, R8::C,  5);}
        0xAA => {res_r8_b3(soc, R8::D,  5);}
        0xAB => {res_r8_b3(soc, R8::E,  5);}
        0xAC => {res_r8_b3(soc, R8::H,  5);}
        0xAD => {res_r8_b3(soc, R8::L,  5);}
        0xAE => {res_r8_b3(soc, R8::HL, 5);}
        0xAF => {res_r8_b3(soc, R8::A,  5);}
        0xB0 => {res_r8_b3(soc, R8::B,  6);}
        0xB1 => {res_r8_b3(soc, R8::C,  6);}
        0xB2 => {res_r8_b3(soc, R8::D,  6);}
        0xB3 => {res_r8_b3(soc, R8::E,  6);}
        0xB4 => {res_r8_b3(soc, R8::H,  6);}
        0xB5 => {res_r8_b3(soc, R8::L,  6);}
        0xB6 => {res_r8_b3(soc, R8::HL, 6);}
        0xB7 => {res_r8_b3(soc, R8::A,  6);}
        0xB8 => {res_r8_b3(soc, R8::B,  7);}
        0xB9 => {res_r8_b3(soc, R8::C,  7);}
        0xBA => {res_r8_b3(soc, R8::D,  7);}
        0xBB => {res_r8_b3(soc, R8::E,  7);}
        0xBC => {res_r8_b3(soc, R8::H,  7);}
        0xBD => {res_r8_b3(soc, R8::L,  7);}
        0xBE => {res_r8_b3(soc, R8::HL, 7);}
        0xBF => {res_r8_b3(soc, R8::A,  7);}
        0xC0 => {set_r8_b3(soc, R8::B,  0);}
        0xC1 => {set_r8_b3(soc, R8::C,  0);}
        0xC2 => {set_r8_b3(soc, R8::D,  0);}
        0xC3 => {set_r8_b3(soc, R8::E,  0);}
        0xC4 => {set_r8_b3(soc, R8::H,  0);}
        0xC5 => {set_r8_b3(soc, R8::L,  0);}
        0xC6 => {set_r8_b3(soc, R8::HL, 0);}
        0xC7 => {set_r8_b3(soc, R8::A,  0);}
        0xC8 => {set_r8_b3(soc, R8::B,  1);}
        0xC9 => {set_r8_b3(soc, R8::C,  1);}
        0xCA => {set_r8_b3(soc, R8::D,  1);}
        0xCB => {set_r8_b3(soc, R8::E,  1);}
        0xCC => {set_r8_b3(soc, R8::H,  1);}
        0xCD => {set_r8_b3(soc, R8::L,  1);}
        0xCE => {set_r8_b3(soc, R8::HL, 1);}
        0xCF => {set_r8_b3(soc, R8::A,  1);}
        0xD0 => {set_r8_b3(soc, R8::B,  2);}
        0xD1 => {set_r8_b3(soc, R8::C,  2);}
        0xD2 => {set_r8_b3(soc, R8::D,  2);}
        0xD3 => {set_r8_b3(soc, R8::E,  2);}
        0xD4 => {set_r8_b3(soc, R8::H,  2);}
        0xD5 => {set_r8_b3(soc, R8::L,  2);}
        0xD6 => {set_r8_b3(soc, R8::HL, 2);}
        0xD7 => {set_r8_b3(soc, R8::A,  2);}
        0xD8 => {set_r8_b3(soc, R8::B,  3);}
        0xD9 => {set_r8_b3(soc, R8::C,  3);}
        0xDA => {set_r8_b3(soc, R8::D,  3);}
        0xDB => {set_r8_b3(soc, R8::E,  3);}
        0xDC => {set_r8_b3(soc, R8::H,  3);}
        0xDD => {set_r8_b3(soc, R8::L,  3);}
        0xDE => {set_r8_b3(soc, R8::HL, 3);}
        0xDF => {set_r8_b3(soc, R8::A,  3);}
        0xE0 => {set_r8_b3(soc, R8::B,  4);}
        0xE1 => {set_r8_b3(soc, R8::C,  4);}
        0xE2 => {set_r8_b3(soc, R8::D,  4);}
        0xE3 => {set_r8_b3(soc, R8::E,  4);}
        0xE4 => {set_r8_b3(soc, R8::H,  4);}
        0xE5 => {set_r8_b3(soc, R8::L,  4);}
        0xE6 => {set_r8_b3(soc, R8::HL, 4);}
        0xE7 => {set_r8_b3(soc, R8::A,  4);}
        0xE8 => {set_r8_b3(soc, R8::B,  5);}
        0xE9 => {set_r8_b3(soc, R8::C,  5);}
        0xEA => {set_r8_b3(soc, R8::D,  5);}
        0xEB => {set_r8_b3(soc, R8::E,  5);}
        0xEC => {set_r8_b3(soc, R8::H,  5);}
        0xED => {set_r8_b3(soc, R8::L,  5);}
        0xEE => {set_r8_b3(soc, R8::HL, 5);}
        0xEF => {set_r8_b3(soc, R8::A,  5);}
        0xF0 => {set_r8_b3(soc, R8::B,  6);}
        0xF1 => {set_r8_b3(soc, R8::C,  6);}
        0xF2 => {set_r8_b3(soc, R8::D,  6);}
        0xF3 => {set_r8_b3(soc, R8::E,  6);}
        0xF4 => {set_r8_b3(soc, R8::H,  6);}
        0xF5 => {set_r8_b3(soc, R8::L,  6);}
        0xF6 => {set_r8_b3(soc, R8::HL, 6);}
        0xF7 => {set_r8_b3(soc, R8::A,  6);}
        0xF8 => {set_r8_b3(soc, R8::B,  7);}
        0xF9 => {set_r8_b3(soc, R8::C,  7);}
        0xFA => {set_r8_b3(soc, R8::D,  7);}
        0xFB => {set_r8_b3(soc, R8::E,  7);}
        0xFC => {set_r8_b3(soc, R8::H,  7);}
        0xFD => {set_r8_b3(soc, R8::L,  7);}
        0xFE => {set_r8_b3(soc, R8::HL, 7);}
        0xFF => {set_r8_b3(soc, R8::A,  7);}
    }
}

// 注册到模块
#[pymodule]
fn sm83_kernel(_py: Python<'_>, m: &PyModule) -> PyResult<()> {m.add_class::<SoC>()?; Ok(())}


// ===========================================================
enum R8 {B, C, D, E, H, L, HL, A}
enum R16 {BC, DE, HL, SP}
enum R16STK {BC, DE, HL, AF}
enum R16RAM {BC, DE, HLI, HLD}
enum COND {NZ, Z, NC, C}
enum TGT3 {T0=0x00, T1=0x08, T2=0x10, T3=0x18, T4=0x20, T5=0x28, T6=0x30, T7=0x38}
enum ALU3 {ADD, ADC, SUB, SBC, AND, XOR, OR, CP}
enum BOP3 {RLC, RRC, RL, RR, SLA, SRA, SWAP, SRL}

fn get_r8_by_idx(soc: &SoC, r8: &R8) -> u8 {
    match r8 {
        R8::B  => {soc.get_r8(2)}
        R8::C  => {soc.get_r8(3)}
        R8::D  => {soc.get_r8(4)}
        R8::E  => {soc.get_r8(5)}
        R8::H  => {soc.get_r8(6)}
        R8::L  => {soc.get_r8(7)}
        R8::HL => {soc.ram_read(soc.get_r16((6, 7)))}
        R8::A  => {soc.get_r8(0)}
    }
}
fn get_r16_by_idx(soc: &SoC, r16: R16) -> u16 {
    match r16 {
        R16::BC => {soc.get_r16((2, 3))},
        R16::DE => {soc.get_r16((4, 5))},
        R16::HL => {soc.get_r16((6, 7))},
        R16::SP => {soc.get_sp()}
    }
}
fn get_r16stk_by_idx(soc: &SoC, r16stk: R16STK) -> u16 {
    match r16stk {
        R16STK::BC => {soc.get_r16((2, 3))},
        R16STK::DE => {soc.get_r16((4, 5))},
        R16STK::HL => {soc.get_r16((6, 7))},
        R16STK::AF => {soc.get_r16((0, 1))},
    }
}
fn get_cond_by_idx(soc: &SoC, cond: COND) -> bool {
    match cond {
        COND::NZ => {soc.get_flag(7) == 0},
        COND::Z  => {soc.get_flag(7) == 1},
        COND::NC => {soc.get_flag(4) == 0},
        COND::C  => {soc.get_flag(4) == 1},
    }
}

fn set_r8_by_idx(soc: &mut SoC, r8: &R8, r8_value: u8) {
    match r8 {
        R8::B  => {soc.set_r8(2, r8_value);}
        R8::C  => {soc.set_r8(3, r8_value);}
        R8::D  => {soc.set_r8(4, r8_value);}
        R8::E  => {soc.set_r8(5, r8_value);}
        R8::H  => {soc.set_r8(6, r8_value);}
        R8::L  => {soc.set_r8(7, r8_value);}
        R8::HL => {soc.ram_write(soc.get_r16((6, 7)),r8_value);}
        R8::A  => {soc.set_r8(0, r8_value);}
    }
}

// ===========================================================
// 其他辅助函数
fn b8x2_le(b8a: u16, b8b: u16) -> u16 {(b8b << 8) + b8a}
fn b8x2(b8a: u16, b8b: u16) -> u16 {(b8a << 8) + b8b}

// ld_r16ram_a
fn ld_r16ram_a(soc: &mut SoC, r16ram: R16RAM) {
    let addr: u16 = match r16ram {
        R16RAM::BC  => {soc.get_r16((2, 3))},
        R16RAM::DE  => {soc.get_r16((4, 5))},
        _           => {soc.get_r16((6, 7))},};
    match r16ram {
        R16RAM::HLI => {soc.r16_inc((6, 7))},
        R16RAM::HLD => {soc.r16_dec((6, 7))},
        _           => {}}
    soc.ram_write(addr, soc.get_r8(0));
    soc.pc_inc(1);
    soc.cyc_inc(2);
}
// inc_r16
fn inc_r16(soc: &mut SoC, r16: R16) {
    match r16 {
        R16::BC => {soc.r16_inc((2, 3))},
        R16::DE => {soc.r16_inc((4, 5))},
        R16::HL => {soc.r16_inc((6, 7))},
        R16::SP => {soc.set_sp(soc.get_sp() + 1);}}
    soc.pc_inc(1);
    soc.cyc_inc(2);
}
// add_hl_r16
fn add_hl_r16(soc: &mut SoC, r16: R16) {
    let add1 = soc.get_r16((6, 7));
    let add2 = get_r16_by_idx(soc, r16);
    soc.res_flag(6);
    if (add1 & 0xfff) + (add2 & 0xfff) > 0xfff {soc.set_flag(5);} else {soc.res_flag(5);};
    if add1 as u32 + add2 as u32 > 0xffff {soc.set_flag(4);} else {soc.res_flag(4);};
    soc.set_r16((6, 7), add1 + add2);
    soc.pc_inc(1);
    soc.cyc_inc(2);
}
// ld_a_r16ram
fn ld_a_r16ram(soc: &mut SoC, r16ram: R16RAM) {
    let addr: u16 = match r16ram {
        R16RAM::BC  => {soc.get_r16((2, 3))},
        R16RAM::DE  => {soc.get_r16((4, 5))},
        _           => {soc.get_r16((6, 7))},};
    match r16ram {
        R16RAM::HLI => {soc.r16_inc((6, 7))},
        R16RAM::HLD => {soc.r16_dec((6, 7))},
        _           => {}}
    soc.set_r8(0, soc.ram_read(addr));
    soc.pc_inc(1);
    soc.cyc_inc(2);
}
// dec_r16
fn dec_r16(soc: &mut SoC, r16: R16) {
    match r16 {
        R16::BC => {soc.r16_dec((2, 3))},
        R16::DE => {soc.r16_dec((4, 5))},
        R16::HL => {soc.r16_dec((6, 7))},
        R16::SP => {soc.set_sp(soc.get_sp() - 1);}}
    soc.pc_inc(1);
    soc.cyc_inc(2);
}
// inc_r8
fn inc_r8(soc: &mut SoC, r8: R8) {
    let r8_value = get_r8_by_idx(soc, &r8);
    set_r8_by_idx(soc, &r8, r8_value + 1);
    if r8_value + 1 == 0 {soc.set_flag(7);} else {soc.res_flag(7);}
    soc.res_flag(6);
    if (r8_value & 0xf) + 1 > 0xf {soc.set_flag(5);} else {soc.res_flag(5);}
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// dec_r8
fn dec_r8(soc: &mut SoC, r8: R8) {
    let r8_value = get_r8_by_idx(soc, &r8);
    set_r8_by_idx(soc, &r8, r8_value - 1);
    if r8_value - 1 == 0 {soc.set_flag(7);} else {soc.res_flag(7);}
    soc.set_flag(6);
    if ((r8_value & 0xf) as i8 - 1) < 0 {soc.set_flag(5);} else {soc.res_flag(5);}
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// ld_r8_r8
fn ld_r8_r8(soc: &mut SoC, r8d: R8, r8s: R8) {
    let r8_value = get_r8_by_idx(soc, &r8s);
    set_r8_by_idx(soc, &r8d, r8_value);
    soc.pc_inc(1);
    if (r8s as u8 == 6) | (r8d as u8 == 6) {soc.cyc_inc(2);} else {soc.cyc_inc(1);}
}
fn alu_a_r8(soc: &mut SoC, r8: R8, alu3: ALU3) {
    let r8_value = get_r8_by_idx(soc, &r8);
    match alu3 {
        ALU3::ADD => {
            soc.smart_flag(soc.get_r8(0) as u16, r8_value as u16, false, true);
            soc.set_r8(0, soc.get_r8(0)+r8_value);
        }
        ALU3::ADC => {
            let c_flag = soc.get_flag(4);
            soc.smart_flag(soc.get_r8(0) as u16, (r8_value + c_flag) as u16, false, true);
            soc.set_r8(0, soc.get_r8(0) + r8_value + c_flag);
        }
        ALU3::SUB => {
            soc.smart_flag(soc.get_r8(0) as u16, r8_value as u16, true, true);
            soc.set_r8(0, soc.get_r8(0) - r8_value);
        }
        ALU3::SBC => {
            let c_flag = soc.get_flag(4);
            soc.smart_flag(soc.get_r8(0) as u16, (r8_value + c_flag) as u16, true, true);
            soc.set_r8(0, soc.get_r8(0) - r8_value - c_flag);
        }
        ALU3::AND => {
            if (soc.get_r8(0) & r8_value) == 0 {soc.set_flag(7);} else {soc.res_flag(7);};
            soc.res_flag(6);
            soc.set_flag(5);
            soc.res_flag(4);
            soc.set_r8(0, soc.get_r8(0) & r8_value);
        }
        ALU3::XOR => {
            if (soc.get_r8(0) ^ r8_value) == 0 {soc.set_flag(7);} else {soc.res_flag(7);};
            soc.res_flag(6);
            soc.res_flag(5);
            soc.res_flag(4);
            soc.set_r8(0, soc.get_r8(0) ^ r8_value);
        }
        ALU3::OR => {
            if (soc.get_r8(0) | r8_value) == 0 {soc.set_flag(7);} else {soc.res_flag(7);};
            soc.res_flag(6);
            soc.res_flag(5);
            soc.res_flag(4);
            soc.set_r8(0, soc.get_r8(0) | r8_value);
        }
        ALU3::CP => {
            soc.smart_flag(soc.get_r8(0) as u16, r8_value as u16, true, true);
        }
    }
    soc.pc_inc(1);
    match r8 {R8::HL => {soc.cyc_inc(2);} _ => {soc.cyc_inc(1);}};
}
// ret_cond
fn ret_cond(soc: &mut SoC, cond: COND) {
    if get_cond_by_idx(soc, cond) == true {
        let lo8 = soc.ram_read(soc.get_sp());
        if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() + 1);};
        let hi8 = soc.ram_read(soc.get_sp());
        if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() + 1);};
        soc.set_pc(lo8 as u16 + ((hi8 as u16) << 8));
        soc.cyc_inc(5);
    } else {
        soc.pc_inc(1);
        soc.cyc_inc(2);
    }
}
// pop_r16stk
fn pop_r16stk(soc: &mut SoC, r16stk: R16STK) {
    let lo8 = soc.ram_read(soc.get_sp());
    if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() + 1);};
    let hi8 = soc.ram_read(soc.get_sp());
    if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() + 1);};
    let new_r16 = lo8 as u16 + ((hi8 as u16) << 8);
    match r16stk {
        R16STK::BC => {soc.set_r16((2, 3), new_r16);},
        R16STK::DE => {soc.set_r16((4, 5), new_r16);},
        R16STK::HL => {soc.set_r16((6, 7), new_r16);},
        R16STK::AF => {soc.set_r16((0, 1), new_r16);},}
    soc.cyc_inc(3);
    soc.pc_inc(1);
}
// push_r16stk
fn push_r16stk(soc: &mut SoC, r16stk: R16STK) {
    let new_r16 = get_r16stk_by_idx(soc, r16stk);
    let hi8 = (new_r16 >> 8) as u8;
    let lo8 = new_r16 as u8;
    if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() - 1);}
    soc.ram_write(soc.get_sp(), hi8);
    if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() - 1);}
    soc.ram_write(soc.get_sp(), lo8);
    soc.cyc_inc(4);
    soc.pc_inc(1);
}
// rst_tgt3
fn rst_tgt3(soc: &mut SoC, tgt3: TGT3) {
    soc.pc_inc(tgt3 as u16);
    soc.cyc_inc(4);
}
// ld_r8_n8
fn ld_r8_n8(soc: &mut SoC, r8: R8, n8: u16) {
    let n8 = n8 as u8;
    set_r8_by_idx(soc, &r8, n8);
    soc.pc_inc(2);
    if r8 as u8 == 6 {soc.cyc_inc(3)} else {soc.cyc_inc(2);};
}
// jr_cond_e8
fn jr_cond_e8(soc: &mut SoC, cond: COND, e8: u16) {
    let e8 = e8 as i8;
    if get_cond_by_idx(soc, cond) == true {
        if e8 > 0 {soc.pc_inc(e8 as u16);} else {soc.pc_dec(-e8 as u16);};
        soc.cyc_inc(3);
    } else {
        soc.pc_inc(2);
        soc.cyc_inc(2);
    }
}
// ld_r16_n16
fn ld_r16_n16(soc: &mut SoC, r16: R16, n16: u16) {
    match r16 {
        R16::BC => {soc.set_r16((2, 3), n16);},
        R16::DE => {soc.set_r16((4, 5), n16);},
        R16::HL => {soc.set_r16((6, 7), n16);},
        R16::SP => {soc.set_sp(n16);},
    }
    soc.cyc_inc(3);
    soc.pc_inc(3);
}
// jp_cond_a16
fn jp_cond_a16(soc: &mut SoC, cond:COND, a16: u16) {
    if get_cond_by_idx(soc, cond) == true {
        soc.set_pc(a16);
        soc.cyc_inc(4);
    } else {
        soc.pc_inc(3);
        soc.cyc_inc(3);
    }
}
// call_cond_a16
fn call_cond_a16(soc: &mut SoC, cond:COND, a16: u16) {
    if get_cond_by_idx(soc, cond) == true {
        let new_pc = soc.get_pc() + 3;
        if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() - 1);};
        soc.ram_write(soc.get_sp(), (new_pc >> 8) as u8);
        if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() - 1);};
        soc.ram_write(soc.get_sp(), (new_pc & 0xff) as u8);
        soc.set_pc(a16);
        soc.cyc_inc(6);
    } else {
        soc.pc_inc(3);
        soc.cyc_inc(3);
    }
}
// bop_r8
fn bop_r8(soc: &mut SoC, r8: R8, bop3: BOP3) {
    let r8_value = get_r8_by_idx(soc, &r8);
    let c: u8;
    let res: u8;
    match bop3 {
        BOP3::RLC => {
            c = r8_value >> 7;
            res = ((r8_value << 1) + c) & 0xff;
        }
        BOP3::RRC => {
            c = r8_value & 1;
            res = ((r8_value >> 1) + (c << 7)) & 0xff;
        }
        BOP3::RL => {
            c = r8_value >> 7;
            res = ((r8_value << 1) + soc.get_flag(4)) & 0xff;
        }
        BOP3::RR => {
            c = r8_value & 1;
            res = (r8_value >> 1) + (soc.get_flag(4) << 7);
        }
        BOP3::SLA => {
            c = r8_value >> 7;
            res = (r8_value << 1) & 0xff;
        }
        BOP3::SRA => {
            c = r8_value & 1;
            res = ((r8_value >> 1) & 0xff) + (r8_value & 0x80);
        }
        BOP3::SWAP => {
            c = 42;
            res = (r8_value >> 4) + ((r8_value & 0xf) << 4);
        }
        BOP3::SRL => {
            c = r8_value & 1;
            res = (r8_value >> 1) & 0xff;
        }
    }
    set_r8_by_idx(soc, &r8, res);
    if res == 0 {soc.set_flag(7);} else {soc.res_flag(7);}
    soc.res_flag(6);
    soc.res_flag(5);
    if (bop3 as u8) != 6 {
        if c == 1 {soc.set_flag(4);} 
        if c == 0 {soc.res_flag(4);}
    }
    soc.pc_inc(2);
    if (r8 as u8) == 6 {soc.cyc_inc(4)} else {soc.cyc_inc(2);}
}
// bit_r8_b3
fn bit_r8_b3(soc: &mut SoC, r8: R8, b3: u8) {
    let r8_value = get_r8_by_idx(soc, &r8);
    if r8_value & (1 << b3) == 0 {
        soc.set_flag(7);
    } else {
        soc.res_flag(7);
    }
    soc.res_flag(6);
    soc.set_flag(5);
    soc.pc_inc(2);
    if (r8 as u8) == 6 {soc.cyc_inc(3);} else {soc.cyc_inc(2);}
}
// res_r8_b3
fn res_r8_b3(soc: &mut SoC, r8: R8, b3: u8) {
    let r8_value = get_r8_by_idx(soc, &r8);
    set_r8_by_idx(soc, &r8, r8_value & (0xff - (1 << b3))); 
    soc.pc_inc(2);
    if (r8 as u8) == 6 {soc.cyc_inc(4);} else {soc.cyc_inc(2);}
}
// set_r8_b3
fn set_r8_b3(soc: &mut SoC, r8: R8, b3: u8) {
    let r8_value = get_r8_by_idx(soc, &r8);
    set_r8_by_idx(soc, &r8, r8_value | (0xff - (1 << b3))); 
    soc.pc_inc(2);
    if (r8 as u8) == 6 {soc.cyc_inc(4);} else {soc.cyc_inc(2);}
}
// nop
fn nop(soc: &mut SoC) {
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// rlca
fn rlca(soc: &mut SoC) {
    let a_value = soc.get_r8(0);
    let c = a_value >> 7;
    soc.set_r8(0, ((a_value << 1) + c) & 0xff);
    soc.res_flag(7);
    soc.res_flag(6);
    soc.res_flag(5);
    if c == 1 {soc.set_flag(4);} 
    if c == 0 {soc.res_flag(4);}
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// rrca
fn rrca(soc: &mut SoC) {
    let a_value = soc.get_r8(0);
    let c = a_value & 1;
    soc.set_r8(0, ((a_value >> 1) + (c << 7)) & 0xff);
    soc.res_flag(7);
    soc.res_flag(6);
    soc.res_flag(5);
    if c == 1 {soc.set_flag(4);} 
    if c == 0 {soc.res_flag(4);}
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// rla
fn rla(soc: &mut SoC) {
    let a_value = soc.get_r8(0);
    let c = a_value >> 7;
    soc.set_r8(0, ((a_value << 1) + soc.get_flag(4)) & 0xff);
    soc.res_flag(7);
    soc.res_flag(6);
    soc.res_flag(5);
    if c == 1 {soc.set_flag(4);} 
    if c == 0 {soc.res_flag(4);}
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// rra
fn rra(soc: &mut SoC) {
    let a_value = soc.get_r8(0);
    let c = a_value & 1;
    soc.set_r8(0, ((a_value >> 1) + (soc.get_flag(4) << 7)) & 0xff);
    soc.res_flag(7);
    soc.res_flag(6);
    soc.res_flag(5);
    if c == 1 {soc.set_flag(4);} 
    if c == 0 {soc.res_flag(4);}
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// daa
fn daa(soc: &mut SoC) {
    let a_value = soc.get_r8(0);
    let res;
    if soc.get_flag(6) == 1 {
        let adj = soc.get_flag(5) * 0x6 + soc.get_flag(4) * 0x60;
        res = a_value - adj;
        soc.smart_flag(a_value as u16, adj as u16, true, true);
    } else {
        let mut adj: u8 = 0x0;
		if (soc.get_flag(5) == 1) || ((a_value & 0xf) > 0x9) {adj = adj + 0x6}
		if (soc.get_flag(4) == 1) || ((a_value & 0x99) > 0x9) {adj = adj + 0x60}
        res = a_value + adj;
        soc.smart_flag(a_value as u16, adj as u16, false, true);
    }
    soc.res_flag(5);
    soc.set_r8(0, res);
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// scf
fn scf(soc: &mut SoC) {
    soc.res_flag(6);
    soc.res_flag(5);
    soc.set_flag(4);
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// cpl
fn cpl(soc: &mut SoC) {
    soc.set_r8(0, soc.get_r8(0) ^ 0xff);
    soc.set_flag(6);
    soc.set_flag(5);
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// ccf
fn ccf(soc: &mut SoC) {
    let c = soc.get_flag(4);
    if c == 1 {soc.res_flag(4);}
    if c == 0 {soc.set_flag(4);}
    soc.res_flag(6);
    soc.res_flag(5);
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// ret
fn ret(soc: &mut SoC) {
    let lo8 = soc.ram_read(soc.get_sp());
    if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() + 1);}
    let hi8 = soc.ram_read(soc.get_sp());
    if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() + 1);}
    soc.set_pc(lo8 as u16 + ((hi8 as u16) << 8));
    soc.cyc_inc(4);
}
// jp_hl
fn jp_hl(soc: &mut SoC) {
    soc.set_pc(soc.get_r16((6, 7)));
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// di
fn di(soc: &mut SoC) {
    soc.res_ime();
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// ei
fn ei(soc: &mut SoC) {
    soc.set_ime();
    soc.pc_inc(1);
    soc.cyc_inc(1);
}
// reti
fn reti(soc: &mut SoC) {
    soc.set_ime();
    let lo8 = soc.ram_read(soc.get_sp());
    if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() + 1);}
    let hi8 = soc.ram_read(soc.get_sp());
    if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp() + 1);}
    soc.set_pc(lo8 as u16 + ((hi8 as u16) << 8));
    soc.cyc_inc(4);
}
// ld_sp_hl
fn ld_sp_hl(soc: &mut SoC) {
    soc.set_sp(soc.get_r16((6, 7)));
    soc.pc_inc(1);
    soc.cyc_inc(2);
}
// jr_e8
fn jr_e8(soc: &mut SoC, e8: i8) {
    let res = (soc.get_pc() as i32 + e8 as i32)as u16;
    soc.set_pc(res);
    soc.cyc_inc(3);
}
// ldh_a8_a
fn ldh_a8_a(soc: &mut SoC, a8: u8) {
    soc.ram_write(0xff00 + (a8 as u16), soc.get_r8(0));
    soc.pc_inc(2);
    soc.cyc_inc(3);
}
// ldh_a_a8
fn ldh_a_a8(soc: &mut SoC, a8: u8) {
    soc.set_r8(0, soc.ram_read(0xff00 + (a8 as u16)));
    soc.pc_inc(2);
    soc.cyc_inc(3);
}
// ldh_c_a
fn ldh_c_a(soc: &mut SoC) {
    soc.ram_write(0xff00 + (soc.get_r8(3) as u16), soc.get_r8(0));
    soc.pc_inc(1);
    soc.cyc_inc(2);
}
// ldh_a_c
fn ldh_a_c(soc: &mut SoC) {
    soc.set_r8(0, soc.ram_read(0xff00 + (soc.get_r8(3) as u16)));
    soc.pc_inc(1);
    soc.cyc_inc(2);
}
// alu_a_n8
fn alu_a_n8(soc: &mut SoC, n8: u16, alu3: ALU3) {
    match alu3 {
        ALU3::ADD => {
            soc.smart_flag(soc.get_r8(0) as u16, n8 as u16, false, true);
            soc.set_r8(0, soc.get_r8(0) + n8 as u8);
        }
        ALU3::ADC => {
            let c_flag = soc.get_flag(4);
            soc.smart_flag(soc.get_r8(0) as u16, (n8 as u8 + c_flag) as u16, false, true);
            soc.set_r8(0, soc.get_r8(0) + n8 as u8 + c_flag);
        }
        ALU3::SUB => {
            soc.smart_flag(soc.get_r8(0) as u16, n8 as u16, true, true);
            soc.set_r8(0, soc.get_r8(0) - n8 as u8);
        }
        ALU3::SBC => {
            let c_flag = soc.get_flag(4);
            soc.smart_flag(soc.get_r8(0) as u16, (n8 as u8 + c_flag) as u16, true, true);
            soc.set_r8(0, soc.get_r8(0) - n8 as u8 - c_flag);
        }
        ALU3::AND => {
            if (soc.get_r8(0) & n8 as u8) == 0 {soc.set_flag(7);} else {soc.res_flag(7);};
            soc.res_flag(6);
            soc.set_flag(5);
            soc.res_flag(4);
            soc.set_r8(0, soc.get_r8(0) & n8 as u8);
        }
        ALU3::XOR => {
            if (soc.get_r8(0) ^ n8 as u8) == 0 {soc.set_flag(7);} else {soc.res_flag(7);};
            soc.res_flag(6);
            soc.res_flag(5);
            soc.res_flag(4);
            soc.set_r8(0, soc.get_r8(0) ^ n8 as u8);
        }
        ALU3::OR => {
            if (soc.get_r8(0) | n8 as u8) == 0 {soc.set_flag(7);} else {soc.res_flag(7);};
            soc.res_flag(6);
            soc.res_flag(5);
            soc.res_flag(4);
            soc.set_r8(0, soc.get_r8(0) | n8 as u8);
        }
        ALU3::CP => {
            soc.smart_flag(soc.get_r8(0) as u16, n8 as u16, true, true);
        }
    }
    soc.pc_inc(1);
    soc.cyc_inc(2);
}
// add_sp_e8
fn add_sp_e8(soc: &mut SoC, e8: i8) {
    let sp = soc.get_sp() as i32;
    let e8 = e8 as i32;
    soc.res_flag(7);
    soc.res_flag(6);
    if e8 > 0 {
        if (sp & 0xfff) + e8 > 0xfff {soc.set_flag(5);} else {soc.res_flag(5);};
        if (sp & 0xffff) + e8 > 0xffff {soc.set_flag(4);} else {soc.res_flag(4);};
        soc.set_sp((sp + e8) as u16);
    } else {
        soc.res_flag(5);
        soc.res_flag(4);
    };
    soc.set_sp((sp + e8) as u16);
    soc.pc_inc(2);
    soc.cyc_inc(4);
}
// ld_hl_sp_e8
fn ld_hl_sp_e8(soc: &mut SoC, e8: i8) {
    let sp = soc.get_sp() as i32;
    let e8 = e8 as i32;
    soc.res_flag(7);
    soc.res_flag(6);
    if e8 > 0 {
        if (sp & 0xfff) + e8 > 0xfff {soc.set_flag(5);} else {soc.res_flag(5);};
        if (sp & 0xffff) + e8 > 0xffff {soc.set_flag(4);} else {soc.res_flag(4);};
        soc.set_sp((sp + e8) as u16);
    } else {
        soc.res_flag(5);
        soc.res_flag(4);
    };
    soc.set_r16((6, 7), (sp + e8) as u16);
    soc.pc_inc(2);
    soc.cyc_inc(3);
}
// ld_a16_sp
fn ld_a16_sp(soc: &mut SoC, a16: u16) {
    soc.ram_write(a16, soc.get_sp() as u8);
    soc.ram_write(a16 + 1, (soc.get_sp() >> 8) as u8);
    soc.pc_inc(3);
    soc.cyc_inc(5);
}
// jp_a16
fn jp_a16(soc: &mut SoC, a16: u16) {
    soc.set_pc(a16);
    soc.cyc_inc(4);
}
// call_a16
fn call_a16(soc: &mut SoC, a16: u16) {
    let pc = soc.get_pc() + 3;
    if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp()-1);}
    soc.ram_write(soc.get_sp(), (pc >> 8) as u8);
    if soc.get_sp() != 0xfffe {soc.set_sp(soc.get_sp()-1);}
    soc.ram_write(soc.get_sp(), pc as u8);
    soc.set_pc(a16);
    soc.cyc_inc(6);
}
// ld_a16_a
fn ld_a16_a(soc: &mut SoC, a16: u16) {
    soc.ram_write(a16, soc.get_r8(0));
    soc.pc_inc(3);
    soc.cyc_inc(4);
}
// ld_a_a16
fn ld_a_a16(soc: &mut SoC, a16: u16) {
    soc.set_r8(0, soc.ram_read(a16));
    soc.pc_inc(3);
    soc.cyc_inc(4);
}
// halt
fn halt(_soc: &mut SoC) {
    /* 此处没有代码 */
}
// stop
fn stop(soc: &mut SoC) {
    soc.pc_inc(2);
    soc.cyc_inc(1);
}

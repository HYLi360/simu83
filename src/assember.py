import argparse
import re

def transformer(open_file_name: str):
	"""transform the asm file to a list"""
	l0 = {
		"NOP": 0,
		"RLCA": 7,
		"RRCA": 15,
		"RLA": 23,
		"RRA": 31,
		"DAA": 39,
		"CPL": 47,
		"SCF": 55,
		"CCF": 63,
		"HALT": 118,
		"RET": 201,
		"RETI": 217,
		"DI": 243,
		"EI": 251,
	}
	l1 = {
		"JP HL": 233,
		"LDH [C], A": 226,
		"LDH A, [C]": 242,
		"LD SP, HL": 249
	}
	opt = {
		"R8": ["B", "C", "D", "E", "H", "L", r"\[HL\]", "A"],  # r8
		"R16": ["BC", "DE", "HL", "SP"],  # r16
		"R16STK": ["BC", "DE", "HL", "AF"],  # r16stk
		"R16RAM": [r"\[BC\]", r"\[DE\]", r"\[HL\+\]", r"\[HL\-\]"],  # r16ram
		"COND": ["NZ", "Z", "NC", "C"],  # cond
		"B3": ["0", "1", "2", "3", "4", "5", "6", "7"],  # bidx
		"TGT3": ["0x00", "0x08", "0x10", "0x18", "0x20", "0x28", "0x30", "0x38"],  # tgt3
		"ALU": ["ADD", "ADC", "SUB", "SBC", "AND", "XOR", "OR", "CP"],  # alu
		"BOP": ["RLC", "RRC", "RL", "RR", "SLA", "SRA", "SWAP", "SRL"],  # bop
	}
	R8 = {"B": 0, "C": 1, "D": 2, "E": 3, "H": 4, "L": 5, "[HL]": 6, "A": 7}
	R16 = {"BC": 0<<4, "DE": 1<<4, "HL": 2<<4, "SP": 3<<4}
	R16STK = {"BC": 0<<4, "DE": 1<<4, "HL": 2<<4, "AF": 3<<4}
	R16RAM = {"[BC]": 0<<4, "[DE]": 1<<4, "[HL+]": 2<<4, "[HL-]": 3<<4}
	COND = {"NZ": 0<<3, "Z": 1<<3, "NC": 2<<3, "C": 3<<3}
	B3 = {"0": 0<<3, "1": 1<<3, "2": 2<<3, "3": 3<<3, "4": 4<<3, "5": 5<<3, "6": 6<<3, "7": 7<<3}
	TGT3 = {"0x00": 0<<3, "0x08": 1<<3, "0x10": 2<<3, "0x18": 3<<3, "0x20": 4<<3, "0x28": 5<<3, "0x30": 6<<3, "0x38": 7<<3}
	ALU = {"ADD": 0<<3, "ADC": 1<<3, "SUB": 2<<3, "SBC": 3<<3, "AND": 4<<3, "XOR": 5<<3, "OR": 6<<3, "CP": 7<<3}
	BOP = {"RLC": 0<<3, "RRC": 1<<3, "RL": 2<<3, "RR": 3<<3, "SLA": 4<<3, "SRA": 5<<3, "SWAP": 6<<3, "SRL": 7<<3}

	pattern_ld_r16ram_a = rf"LD\s({('|'.join(opt['R16RAM']))}),\sA"
	pattern_ld_a_r16ram = rf"LD\sA,\s({('|'.join(opt['R16RAM']))})"
	pattern_alu_a_r8 = rf"({('|'.join(opt['ALU']))})\sA,\s({('|'.join(opt['R8']))})"
	pattern_inc_r16 = rf"INC\s({('|'.join(opt['R16']))})"
	pattern_dec_r16 = rf"DEC\s({('|'.join(opt['R16']))})"
	pattern_inc_r8 = rf"INC\s({('|'.join(opt['R8']))})"
	pattern_dec_r8 = rf"DEC\s({('|'.join(opt['R8']))})"
	pattern_add_hl_r16 = rf"ADD\s+HL,\s({('|'.join(opt['R16']))})"
	pattern_ld_r8_r8 = rf"LD\s({('|'.join(opt['R8']))}),\s({('|'.join(opt['R8']))})"
	pattern_ret_cond = rf"RET\s({('|'.join(opt['COND']))})"
	pattern_pop_r16stk = rf"POP\s({('|'.join(opt['R16STK']))})"
	pattern_push_r16stk = rf"PUSH\s({('|'.join(opt['R16STK']))})"
	pattern_rst_tgt3 = rf"RST\s({('|'.join(opt['TGT3']))})"
	pattern_bop_r8 = rf"({('|'.join(opt['BOP']))})\s({('|'.join(opt['R8']))})"
	pattern_bit_b3_r8 = rf"BIT\s({('|'.join(opt['B3']))}),\s({('|'.join(opt['R8']))})"
	pattern_res_b3_r8 = rf"RES\s({('|'.join(opt['B3']))}),\s({('|'.join(opt['R8']))})"
	pattern_set_b3_r8 = rf"SET\s({('|'.join(opt['B3']))}),\s({('|'.join(opt['R8']))})"
	pattern_ld_r8_n8 = rf"LD\s({('|'.join(opt['R8']))}),\s(0X[0-9A-F]+|\d+)"
	pattern_jr_cond_e8 = rf"JR\s({('|'.join(opt['COND']))}),\s([-+]?\d+)"
	pattern_ld_r16_n16 = rf"LD\s({('|'.join(opt['R16']))}),\s(0X[0-9A-F]+|\d+)"
	pattern_jp_cond_a16 = rf"JP\s({('|'.join(opt['COND']))}),\s(0X[0-9A-F]+|\d+)"
	pattern_call_cond_a16 = rf"CALL\s({('|'.join(opt['COND']))}),\s(0X[0-9A-F]+|\d+)"
	pattern_stop = rf"STOP\s(0X[0-9A-F]+|\d+)"
	pattern_jr_e8 = rf"JR\s([-+]?\d+)"
	pattern_alu_a_n8 = rf"({('|'.join(opt['ALU']))})\sA,\s(0X[0-9A-F]+|\d+)"
	pattern_add_sp_e8 = rf"ADD\sSP,\s([-+]?\d+)"
	pattern_ldh_a8_a = rf"LDH\s\[(0X[0-9A-F]+|\d+)\],\sA"
	pattern_ldh_a_a8 = rf"LDH\sA,\s\[(0X[0-9A-F]+|\d+)\]"
	pattern_ld_hl_sp_e8 = rf"LD\sHL,\sSP\s*([-+]?\d+)"
	pattern_ld_a16_sp = rf"LD\s\[(0X[0-9A-F]+|\d+)\],\sSP"
	pattern_jp_a16 = rf"JP\s(0X[0-9A-F]+|\d+)"
	pattern_call_a16 = rf"CALL\s(0X[0-9A-F]+|\d+)"
	pattern_ld_a16_a = rf"LD\s\[(0X[0-9A-F]+|\d+)\],\sA"
	pattern_ld_a_a16 = rf"LD\sA,\s\[(0X[0-9A-F]+|\d+)\]"

	inst_list = []
	res = b""

	with open(open_file_name, "r") as f1:
		for line in f1:
			new_line = line.strip().upper()
			inst_list.append(new_line)


	for new_line in inst_list:
		if new_line in l0:
			res = res + bytes([l0[new_line]])
		elif new_line in l1:
			res = res + bytes([l1[new_line]])
		else:
			ld_r16ram_a = re.search(pattern_ld_r16ram_a, new_line)
			if ld_r16ram_a is not None:
				res = res + bytes([R16RAM[ld_r16ram_a.group(1)] + 2])
				continue
			ld_a_r16ram = re.search(pattern_ld_a_r16ram, new_line)
			if ld_a_r16ram is not None:
				res = res + bytes([R16RAM[ld_a_r16ram.group(1)] + 10])
				continue
			alu_a_r8 = re.search(pattern_alu_a_r8, new_line)
			if alu_a_r8 is not None:
				res = res + bytes([ALU[alu_a_r8.group(1)] + R8[alu_a_r8.group(2)] + 128])
				continue
			inc_r16 = re.search(pattern_inc_r16, new_line)
			if inc_r16 is not None:
				res = res + bytes([R16[inc_r16.group(1)] + 3])
				continue
			dec_r16 = re.search(pattern_dec_r16, new_line)
			if dec_r16 is not None:
				res = res + bytes([R16[dec_r16.group(1)] + 11])
				continue
			inc_r8 = re.search(pattern_inc_r8, new_line)
			if inc_r8 is not None:
				res = res + bytes([(R8[inc_r8.group(1)] << 3) + 4])
				continue
			dec_r8 = re.search(pattern_dec_r8, new_line)
			if dec_r8 is not None:
				res = res + bytes([(R8[dec_r8.group(1)] << 3) + 5])
				continue
			add_hl_r16 = re.search(pattern_add_hl_r16, new_line)
			if add_hl_r16 is not None:
				res = res + bytes([R16[add_hl_r16.group(1)] + 9])
				continue
			ld_r8_r8 = re.search(pattern_ld_r8_r8, new_line)
			if ld_r8_r8 is not None:
				res = res + bytes([(R8[ld_r8_r8.group(1)] << 3) + R8[ld_r8_r8.group(2)] + 64])
				continue
			ret_cond = re.search(pattern_ret_cond, new_line)
			if ret_cond is not None:
				res = res + bytes([COND[ret_cond.group(1)] + 192])
				continue
			pop_r16stk = re.search(pattern_pop_r16stk, new_line)
			if pop_r16stk is not None:
				res = res + bytes([R16STK[pop_r16stk.group(1)] + 193])
				continue
			push_r16stk = re.search(pattern_push_r16stk, new_line)
			if push_r16stk is not None:
				res = res + bytes([R16STK[push_r16stk.group(1)] + 197])
				continue
			rst_tgt3 = re.search(pattern_rst_tgt3, new_line)
			if rst_tgt3 is not None:
				res = res + bytes([TGT3[rst_tgt3.group(1)] + 199])
				continue

			bop_r8 = re.search(pattern_bop_r8, new_line)
			if bop_r8 is not None:
				res = res + b"\xCB" + bytes([BOP[bop_r8.group(1)] + R8[bop_r8.group(2)]])
				continue
			bit_b3_r8 = re.search(pattern_bit_b3_r8, new_line)
			if bit_b3_r8 is not None:
				res = res + b"\xCB" + bytes([B3[bit_b3_r8.group(1)] + R8[bit_b3_r8.group(2)] + 64])
				continue
			res_b3_r8 = re.search(pattern_res_b3_r8, new_line)
			if res_b3_r8 is not None:
				res = res + b"\xCB" + bytes([B3[res_b3_r8.group(1)] + R8[res_b3_r8.group(2)] + 128])
				continue
			set_b3_r8 = re.search(pattern_set_b3_r8, new_line)
			if set_b3_r8 is not None:
				res = res + b"\xCB" + bytes([B3[set_b3_r8.group(1)] + R8[set_b3_r8.group(2)] + 192])
				continue

			ld_r8_n8 = re.search(pattern_ld_r8_n8, new_line)
			if ld_r8_n8 is not None:
				if ld_r8_n8.group(2)[0:2] == "0X":
					res = res + bytes([(R8[ld_r8_n8.group(1)] << 3) + 6]) + bytes([int(ld_r8_n8.group(2), 16)])
				else:
					res = res + bytes([(R8[ld_r8_n8.group(1)] << 3) + 6]) + bytes([int(ld_r8_n8.group(2), 10)])
				continue
			jr_cond_e8 = re.search(pattern_jr_cond_e8, new_line)
			if jr_cond_e8 is not None:
				if jr_cond_e8.group(2)[0] == "-":
					res = res + bytes([COND[jr_cond_e8.group(1)] + 32]) + (-int(jr_cond_e8.group(2)[1:], 10)).to_bytes(length=1, signed=True)
				else:
					res = res + bytes([COND[jr_cond_e8.group(1)] + 32]) + (int(jr_cond_e8.group(2), 10)).to_bytes(length=1, signed=True)
				continue
			ld_r16_n16 = re.search(pattern_ld_r16_n16, new_line)
			if ld_r16_n16 is not None:
				if ld_r16_n16.group(2)[0:2] == "0X":
					res = res + bytes([R16[ld_r16_n16.group(1)] + 1]) + int(ld_r16_n16.group(2), 16).to_bytes(length=2, byteorder='little')
				else:
					res = res + bytes([R16[ld_r16_n16.group(1)] + 1]) + int(ld_r16_n16.group(2), 10).to_bytes(length=2, byteorder='little')
				continue
			jp_cond_a16 = re.search(pattern_jp_cond_a16, new_line)
			if jp_cond_a16 is not None:
				if jp_cond_a16.group(2)[0:2] == "0X":
					res = res + bytes([COND[jp_cond_a16.group(1)] + 194]) + int(jp_cond_a16.group(2), 16).to_bytes(length=2, byteorder='little')
				else:
					res = res + bytes([COND[jp_cond_a16.group(1)] + 194]) + int(jp_cond_a16.group(2), 10).to_bytes(length=2, byteorder='little')
				continue
			call_cond_a16 = re.search(pattern_call_cond_a16, new_line)
			if call_cond_a16 is not None:
				if call_cond_a16.group(2)[0:2] == "0X":
					res = res + bytes([COND[call_cond_a16.group(1)] + 196]) + int(call_cond_a16.group(2), 16).to_bytes(length=2, byteorder='little')
				else:
					res = res + bytes([COND[call_cond_a16.group(1)] + 196]) + int(call_cond_a16.group(2), 10).to_bytes(length=2, byteorder='little')
				continue
			stop = re.search(pattern_stop, new_line)
			if stop is not None:
				if stop.group(1)[0:2] == "0X":
					res = res + b"\x10" + int(call_cond_a16.group(1), 16).to_bytes(length=1)
				else:
					res = res + b"\x10" + int(call_cond_a16.group(1), 10).to_bytes(length=1)
				continue
			jr_e8 = re.search(pattern_jr_e8, new_line)
			if jr_e8 is not None:
				if jr_e8.group(1)[0] == "-":
					res = res + b"\x18" + (-int(jr_e8.group(1)[1:], 10)).to_bytes(length=1, signed=True)
				else:
					res = res + b"\x18" + (int(jr_e8.group(1), 10)).to_bytes(length=1, signed=True)
				continue
			alu_a_n8 = re.search(pattern_alu_a_n8, new_line)
			if alu_a_n8 is not None:
				if alu_a_n8.group(2)[0:2] == "0X":
					res = res + bytes([ALU[alu_a_n8.group(1)] + 198]) + bytes([int(alu_a_n8.group(2), 16)])
				else:
					res = res + bytes([ALU[alu_a_n8.group(1)] + 198]) + bytes([int(alu_a_n8.group(2), 10)])
				continue
			add_sp_e8 = re.search(pattern_add_sp_e8, new_line)
			if add_sp_e8 is not None:
				if add_sp_e8.group(1)[0] == "-":
					res = res + b"\xE8" + (-int(add_sp_e8.group(1)[1:], 10)).to_bytes(length=1, signed=True)
				else:
					res = res + b"\xE8" + (int(add_sp_e8.group(1), 10)).to_bytes(length=1, signed=True)
				continue
			ldh_a8_a = re.search(pattern_ldh_a8_a, new_line)
			if ldh_a8_a is not None:
				if ldh_a8_a.group(1)[0:2] == "0X":
					res = res + b"\xE0" + bytes([int(ldh_a8_a.group(1), 16)])
				else:
					res = res + b"\xE0" + bytes([int(ldh_a8_a.group(1), 10)])
				continue
			ldh_a_a8 = re.search(pattern_ldh_a_a8, new_line)
			if ldh_a_a8 is not None:
				if ldh_a_a8.group(1)[0:2] == "0X":
					res = res + b"\xF0" + bytes([int(ldh_a_a8.group(1), 16)])
				else:
					res = res + b"\xF0" + bytes([int(ldh_a_a8.group(1), 10)])
				continue
			ld_hl_sp_e8 = re.search(pattern_ld_hl_sp_e8, new_line)
			if ld_hl_sp_e8 is not None:
				if ld_hl_sp_e8.group(1)[0] == "-":
					res = res + b"\xF8" + (-int(ld_hl_sp_e8.group(1)[1:], 10)).to_bytes(length=1, signed=True)
				else:
					res = res + b"\xF8" + (int(ld_hl_sp_e8.group(1), 10)).to_bytes(length=1, signed=True)
				continue
			ld_a16_sp = re.search(pattern_ld_a16_sp, new_line)
			if ld_a16_sp is not None:
				if ld_a16_sp.group(1)[0:2] == "0X":
					res = res + b"\x08" + int(ld_a16_sp.group(1), 16).to_bytes(length=2, byteorder='little')
				else:
					res = res + b"\x08" + int(ld_a16_sp.group(1), 10).to_bytes(length=2, byteorder='little')
			jp_a16 = re.search(pattern_jp_a16, new_line)
			if jp_a16 is not None:
				if jp_a16.group(1)[0:2] == "0X":
					res = res + b"\xC3" + int(jp_a16.group(1), 16).to_bytes(length=2, byteorder='little')
				else:
					res = res + b"\xC3" + int(jp_a16.group(1), 10).to_bytes(length=2, byteorder='little')
				continue
			call_a16 = re.search(pattern_call_a16, new_line)
			if call_a16 is not None:
				if call_a16.group(1)[0:2] == "0X":
					res = res + b"\xCD" + int(call_a16.group(1), 16).to_bytes(length=2, byteorder='little')
				else:
					res = res + b"\xCD" + int(call_a16.group(1), 10).to_bytes(length=2, byteorder='little')
				continue
			ld_a16_a = re.search(pattern_ld_a16_a, new_line)
			if ld_a16_a is not None:
				if ld_a16_a.group(1)[0:2] == "0X":
					res = res + b"\xEA" + int(ld_a16_a.group(1), 16).to_bytes(length=2, byteorder='little')
				else:
					res = res + b"\xEA" + int(ld_a16_a.group(1), 10).to_bytes(length=2, byteorder='little')
				continue
			ld_a_a16 = re.search(pattern_ld_a_a16, new_line)
			if ld_a_a16 is not None:
				if ld_a_a16.group(1)[0:2] == "0X":
					res = res + b"\xFA" + int(ld_a_a16.group(1), 16).to_bytes(length=2, byteorder='little')
				else:
					res = res + b"\xFA" + int(ld_a_a16.group(1), 10).to_bytes(length=2, byteorder='little')
				continue
	return res

def writer(open_file_name: str, write_file_name: str):
	with open(write_file_name, "wb") as f2:
		res = transformer(open_file_name)
		f2.write(res)

def main():
	parser = argparse.ArgumentParser(description="a simple assembler for SM83.")

	parser.add_argument('--input', required=True, help='input .asm file name')
	parser.add_argument('--output', required=True, help='output binary file name')

	args = parser.parse_args()

	writer(open_file_name=args.input, write_file_name=args.output)


"""
if __name__ == "__main__":
	writer()
"""

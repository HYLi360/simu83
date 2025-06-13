import argparse
import sys

def reader_gb(file_name: str):
	"""get hex-list of *.gb (or ROM)"""
	with open(file_name, "rb") as f:
		rom = [int(c) for c in f.read()]
		return rom

def addr(pointer: int):
	return "0x" + format(pointer, "04x") + " "

def b8(data: int):
	return "0x" + format(data, "02x")

def b16(data1: int, data2: int):
	return "0x" + format(data1 + (data2 << 8), "04x")

def signed_int_from_hex(hex_int, bits):
    if hex_int >= (1 << (bits - 1)):
        hex_int -= (1 << bits)
    return hex_int

def signed(data):
	offset = signed_int_from_hex(data, 8)
	return ("+" + str(offset)) if offset > 0 else str(offset)

def transformer(rom_data):
	"""transform the asm file to a list"""
	l0 = {
		0: "nop",
		7: "rlca",
		15: "rrca",
		23: "rla",
		31: "rra",
		39: "daa",
		47: "cpl",
		55: "scf",
		63: "ccf",
		118: "halt",
		201: "ret",
		217: "reti",
		243: "di",
		251: "ei",
		233: "jp hl",
		226: "ldh [c], a",
		242: "ldh a, [c]",
		249: "ld sp, hl",
	}
	l1 = {0x18, 0xe8, 0xe0, 0xf0, 0xf8}
	l2 = {0x08, 0xc3, 0xcd, 0xea, 0xfa}

	R8 = {0: "b", 1: "c", 2: "d", 3: "e", 4: "h", 5: "l", 6: "[hl]", 7: "a"}
	R8_mid = {0<<3: "b", 1<<3: "c", 2<<3: "d", 3<<3: "e", 4<<3: "h", 5<<3: "l", 6<<3: "[hl]", 7<<3: "a"}
	R16 = {0<<4: "bc", 1<<4: "de", 2<<4: "hl", 3<<4: "sp"}
	R16STK = {0<<4: "bc",  1<<4: "de", 2<<4: "hl", 3<<4: "af"}
	R16RAM = {0<<4: "[bc]", 1<<4: "[de]", 2<<4: "[hl+]", 3<<4: "[hl-]"}
	COND = {0<<3: "nz", 1<<3: "z", 2<<3: "nc", 3<<3: "c"}
	B3 = {0<<3: "0", 1<<3: "1", 2<<3: "2", 3<<3: "3", 4<<3: "4", 5<<3: "5", 6<<3: "6", 7<<3: "7"}
	TGT3 = {0<<3: "0x00", 1<<3: "0x08", 2<<3: "0x10", 3<<3: "0x18", 4<<3: "0x20", 5<<3: "0x28", 6<<3: "0x30", 7<<3: "0x38"}
	ALU = {0<<3: "add", 1<<3: "adc", 2<<3: "sub", 3<<3: "sbc", 4<<3: "and", 5<<3: "xor", 6<<3: "or", 7<<3: "cp"}
	BOP = {0<<3: "rlc", 1<<3: "rrc", 2<<3: "rl", 3<<3: "rr", 4<<3: "sla", 5<<3: "sra", 6<<3: "swap", 7<<3: "srl"}

	rom_data = rom_data + [0, 0]
	p = 0

	while p <= (len(rom_data) - 3):
		if rom_data[p] in l0:
			print(addr(p) + l0[rom_data[p]])
			p += 1
			continue
		else:
			op = rom_data[p:p+3]
			if op[0] in l1:
				if op[0] == 0x18:
					print(addr(p) + "jr " + signed(op[1]))
					p += 2
					continue
				elif op[0] == 0xe8:
					print(addr(p) + "add sp, " + signed(op[1]))
					p += 2
					continue
				elif op[0] == 0xe0:
					print(addr(p) + "ldh [" + b8(op[1]) + "], a")
					p += 2
					continue
				elif op[0] == 0xf0:
					print(addr(p) + "ldh a, [" + b8(op[1]) + "]")
					p += 2
					continue
				elif op[0] == 0xf8:
					print(addr(p) + "ld hl, sp" + signed(op[1]))
					p += 2
					continue
				else:
					print(op[0], op[1], op[2])
					p += 1
					continue
			elif op[0] in l2:
				if op[0] == 0x08:
					print(addr(p) + "ld [" + b16(op[1], op[2]) + "], sp")
					p += 3
					continue
				elif op[0] == 0xc3:
					print(addr(p) + "jp " + b16(op[1], op[2]))
					p += 3
					continue
				elif op[0] == 0xcd:
					print(addr(p) + "call " + b16(op[1], op[2]))
					p += 3
					continue
				elif op[0] == 0xea:
					print(addr(p) + "ld [" + b16(op[1], op[2]) + "], a")
					p += 3
					continue
				elif op[0] == 0xfa:
					print(addr(p) + "ld a, [" + b16(op[1], op[2]) + "]")
					p += 3
					continue
				else:
					print(op[0], op[1], op[2])
					p += 1
					continue
			elif op[0] == 0xCB:
				if op[1] >> 6 == 0:
					print(addr(p) + BOP[op[1] & 0x38] + " " + R8[op[1] & 0x07])
					p += 2
					continue
				elif op[1] >> 6 == 1:
					print(addr(p) + "bit " + B3[op[1] & 0x38] + ", " + R8[op[1] & 0x07])
					p += 2
					continue
				elif op[1] >> 6 == 2:
					print(addr(p) + "res " + B3[op[1] & 0x38] + ", " + R8[op[1] & 0x07])
					p += 2
					continue
				elif op[1] >> 6 == 3:
					print(addr(p) + "set " + B3[op[1] & 0x38] + ", " + R8[op[1] & 0x07])
					p += 2
					continue
				else:
					print(op[0], op[1], op[2])
					p += 1
					continue
			elif op[0] >> 6 == 0:
				if op[0] - 2 in R16RAM:
					print(addr(p) + "ld " + R16RAM[op[0] & 0x30] + ", a")
					p += 1
					continue
				elif op[0] - 3 in R16:
					print(addr(p) + "inc " + R16[op[0] & 0x30])
					p += 1
					continue
				elif op[0] - 9 in R16:
					print(addr(p) + "add hl, " + R16[op[0] & 0x30])
					p += 1
					continue
				elif op[0] - 10 in R16RAM:
					print(addr(p) + "ld a, " + R16RAM[op[0] & 0x30])
					p += 1
					continue
				elif op[0] - 11 in R16:
					print(addr(p) + "dec " + R16[op[0] & 0x30])
					p += 1
					continue
				elif op[0] - 4 in R8_mid:
					print(addr(p) + "inc " + R8_mid[op[0] & 0x38])
					p += 1
					continue
				elif op[0] - 5 in R8_mid:
					print(addr(p) + "dec " + R8_mid[op[0] & 0x38])
					p += 1
					continue
				elif op[0] - 6 in R8_mid:
					print(addr(p) + "ld " + R8_mid[op[0] & 0x38] + ", " + b8(op[1]))
					p += 2
					continue
				elif op[0] - 32 in COND:
					print(addr(p) + "jr " + COND[op[0] & 0x18] + ", " + signed(op[1]))
					p += 2
					continue
				elif op[0] - 1 in R16:
					print(addr(p) + "ld " + R16[op[0] & 0x30] + ", " + b16(op[1], op[2]))
					p += 3
					continue
				else:
					print(op[0], op[1], op[2])
					p += 1
					continue
			elif op[0] >> 6 == 1:
				print(addr(p) + "ld " + R8_mid[op[0] & 0x38] + ", " + R8[op[0] & 0x07])
				p += 1
				continue
			elif op[0] >> 6 == 2:
				print(addr(p) + ALU[op[0] & 0x38] + " a, " + R8[op[0] & 0x07])
				p += 1
				continue
			elif op[0] >> 6 == 3:
				if op[0] - 192 in COND:
					print(addr(p) + "ret " + COND[op[0] & 0x18])
					p += 1
					continue
				elif op[0] - 193 in R16STK:
					print(addr(p) + "pop " + R16STK[op[0] & 0x30])
					p += 1
					continue
				elif op[0] - 197 in R16STK:
					print(addr(p) + "push " + R16STK[op[0] & 0x30])
					p += 1
					continue
				elif op[0] - 198 in ALU:
					print(addr(p) + ALU[op[0] & 0x38] + " a, " + b8(op[1]))
					p += 2
					continue
				elif op[0] - 199 in TGT3:
					print(addr(p) + "rst " + TGT3[op[0] & 0x38])
					p += 1
					continue
				elif op[0] - 194 in COND:
					print(addr(p) + "jp " + COND[op[0] & 0x18] + ", " + b16(op[1], op[2]))
					p += 3
					continue
				elif op[0] - 196 in COND:
					print(addr(p) + "call " + COND[op[0] & 0x18] + ", " + b16(op[1], op[2]))
					p += 3
					continue
				else:
					print(op[0], op[1], op[2])
					p += 1
					continue
			else:
				print(op[0], op[1], op[2])
				p += 1
				continue


def main():
	parser = argparse.ArgumentParser(description="A simple disassembler for SM83.")

	parser.add_argument('--input', required=True, help='input .asm file name')
	parser.add_argument('--output', required=True, help='output binary file name')

	args = parser.parse_args()

	rom_data = reader_gb(args.input)

	o_out = sys.stdout
	with open(args.output, "w") as f:
		sys.stdout = f
		transformer(rom_data)
	sys.stdout = o_out

if __name__ == "__main__":
	main()

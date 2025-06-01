"""
rom_reader.py
用来读取.gb或.bin 文件。
"""

def reader_gb(file_name: str):
	"""get hex-list of *.gb (or ROM)"""
	with open(file_name, "rb") as f:
		rom = tuple([int(c) for c in f.read()])
		return rom

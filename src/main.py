import timeit, simu83
from rom_reader import reader_gb

rom_path = "/home/hyli360/rust_project/sm83_kernel/src/test.bin"
rom = reader_gb(rom_path)

soc = simu83.SoC(rom)

def main():
    while not soc.halt():
        soc.one_step()
        soc.disp()

if __name__ == "__main__":
    main()

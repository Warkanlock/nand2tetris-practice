hardware:
	@echo "Building hardware..."
	./materials/tools/HardwareSimulator.sh

cpu:
	@echo "Opening simulator..."
	./materials/tools/CPUEmulator.sh

asm:
	@echo "Opening assembler simulator..."
	./materials/tools/Assembler.sh

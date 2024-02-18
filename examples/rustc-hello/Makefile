TARGET = hello_rust.elf

OBJS = hello_c.o hello_rust.o

all: rm-elf $(TARGET)

include $(KOS_BASE)/Makefile.rules

%.o: %.rs
	rustc-dc $< -o $@

clean: rm-elf
	-rm -f $(OBJS)

rm-elf:
	-rm -f $(TARGET)

$(TARGET): $(OBJS)
	kos-cc -o $(TARGET) $(OBJS)

run: $(TARGET)
	$(KOS_LOADER) $(TARGET)

dist: $(TARGET)
	-rm -f $(OBJS)
	$(KOS_STRIP) $(TARGET)

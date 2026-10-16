COBC ?= cobc
# banco.cob runs past column 72, so it needs variable-format source.
COBFLAGS ?= -x -fformat=variable

DATA = clientes.dat cuentas.dat tarjetas.dat movimientos.dat config.dat empleados.dat

all: banco intereses

banco: banco.cob
	$(COBC) $(COBFLAGS) -o $@ $<

intereses: intereses.cob
	$(COBC) $(COBFLAGS) -o $@ $<

tests/accept: tests/accept.cob
	$(COBC) $(COBFLAGS) -o $@ $<

# The program fails with status 35 if a data file is missing,
# so create them empty first.
data:
	touch $(DATA)

run: banco data
	./banco

golden: banco intereses tests/accept
	./capture-golden.sh

clean:
	rm -f banco intereses tests/accept

.PHONY: all data run golden clean

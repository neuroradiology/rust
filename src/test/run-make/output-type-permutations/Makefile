-include ../tools.mk

all:
	$(RUSTC) foo.rs --crate-type=rlib,dylib,staticlib -Z print-link-args
	ls -l $(TMPDIR)
	$(call REMOVE_RLIBS,bar)
	$(call REMOVE_DYLIBS,bar)
	rm $(TMPDIR)/$(call STATICLIB_GLOB,bar)
	# Check that $(TMPDIR) is empty.
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --crate-type=bin -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/$(call BIN,bar)
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --emit=asm,ir,bc,obj,link -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/bar.ll
	rm $(TMPDIR)/bar.bc
	rm $(TMPDIR)/bar.s
	rm $(TMPDIR)/bar.o
	rm $(TMPDIR)/$(call BIN,bar)
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --emit=asm -o $(TMPDIR)/foo -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/foo
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --emit=bc -o $(TMPDIR)/foo -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/foo
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --emit=ir -o $(TMPDIR)/foo -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/foo
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --emit=obj -o $(TMPDIR)/foo -Z print-link-args -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/foo
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --emit=link -o $(TMPDIR)/foo -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/$(call BIN,foo)
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --crate-type=rlib -o $(TMPDIR)/foo -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/foo
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --crate-type=dylib -o $(TMPDIR)/foo -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/$(call BIN,foo)  # FIXME 13794
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --crate-type=staticlib -o $(TMPDIR)/foo -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/foo
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --crate-type=bin -o $(TMPDIR)/foo -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/$(call BIN,foo)
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

	$(RUSTC) foo.rs --emit=asm,ir,bc,obj,link --crate-type=staticlib -Z print-link-args
	ls -l $(TMPDIR)
	rm $(TMPDIR)/bar.ll
	rm $(TMPDIR)/bar.s
	rm $(TMPDIR)/bar.o
	rm $(TMPDIR)/$(call STATICLIB_GLOB,bar)
	mv $(TMPDIR)/bar.bc $(TMPDIR)/foo.bc
	# Don't check that the $(TMPDIR) is empty - we left `foo.bc` for later
	# comparison.

	$(RUSTC) foo.rs --emit=bc,link --crate-type=rlib -Z print-link-args
	ls -l $(TMPDIR)
	cmp $(TMPDIR)/foo.bc $(TMPDIR)/bar.bc
	rm $(TMPDIR)/bar.bc
	rm $(TMPDIR)/foo.bc
	$(call REMOVE_RLIBS,bar)
	[ "$$(ls -1 $(TMPDIR) | wc -l)" -eq "0" ]

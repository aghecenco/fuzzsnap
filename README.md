# fuzzsnap

To run the fuzzer:

```bash
INPUTDIR=$PWD/in
OUTPUTDIR=$PWD/out

cargo afl build
AFL_SKIP_CPUFREQ=1 cargo afl fuzz -i $INPUTDIR -o $OUTPUTDIR target/debug/fuzzsnap
```

The fuzzer runs indefinitely. Stop it when you get bored.
The crashes are saved in `$OUTPUTDIR/crashes`.

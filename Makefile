# If there is an error while executing a command to build a target,
# delete the built target to ensure that nothing gets corrupted and that
# the target will be rebuilt the next time make is run.
.DELETE_ON_ERROR:

# Do all commands in a target in a single shell.
.ONESHELL:

# Use bash with the -euo and pipefail options
.SHELL: $(shell which bash) -euo pipefail


# Run benchmarks and store results
bench:
	rm -rf target/criterion
	sudo nice -n -20 su $$USER -l -c "cd $$PWD && nix-shell --run 'cargo bench --workspace'"

	CORES=$$(lscpu --json | jq '.lscpu | map(select(.field == "CPU(s):")) | .[0].data')
	CPU=$$(lscpu --json | jq '.lscpu | map(select(.field == "Model name:")) | .[0].data')
	CPUINFO=$$(jq -n "{cores: $$CORES, cpu: $$CPU}")
	DATE=$$(date --iso-8601=s)
	BENCHES="[]"
	for bench in target/criterion/*; do
		if [[ -e "$$bench/new" ]]; then
			NAME=$$(cat "$$bench/new/benchmark.json" | jq '.title')
			MEAN=$$(cat "$$bench/new/estimates.json" | jq '.mean.point_estimate')
			MEDIAN=$$(cat "$$bench/new/estimates.json" | jq '.median.point_estimate')
			BENCH=$$(jq -n "{name: $$NAME, mean: $$MEAN, median: $$MEDIAN}")
			BENCHES=$$(echo "$$BENCHES" | jq "[.[], $$BENCH]")
		fi
	done
	RECORD=$$(jq -n "{cpuinfo: $$CPUINFO, date: \"$$DATE\", benches: $$BENCHES}")

	if [[ -e benches/history.json ]]; then
		PAST=$$(cat benches/history.json)
	else
		PAST="[]"
	fi

	echo $$PAST | jq --indent 2 "[$$RECORD, .[]]" > benches/history.json

.PHONY: bench

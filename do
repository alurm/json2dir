#!/usr/bin/env -S es --

# For the lack of hierarchical lists in es.
# Stores the list in a closure and returns it when called.
fn list { return @ _ { return $* } }

fn split-by-double-dash {
	let (
		args = <={ list $* }
		rest = <={ list }
		indicies = `{ seq $#* }
	) {
		for (i = $indicies) if { ~ $*($i) -- } {
			args = <= {
				if { ~ $i 1 } { list } {
					list $*(... `{ expr $i - 1 })
				}
			}
			rest = <= {
				list $*(`{expr $i + 1} ...)
			}
			break
		}
		return $args $rest
	}
}

fn help {
	printf 'Available actions:\n'

	for ((name help _) = $actions) {
		printf '\n%s:\n\t%s\n' $name $help
	}

	exit 1
}

# NOTE: "run" must be bound dynamically.
fn run-shell-tests {
	let (
		fn-fail = @ description {
			printf '%s\n' 'Error: a test has failed: '$description
			exit 1
		}
	) {
		echo '{}' | run -- --help > output >[2=1] && fail 'got a zero exit code on usage'
		cat output | grep -qi usage || fail 'no usage'

		echo '{}' | run || fail 'got a non-zero exit code on valid input'

		echo f | run >[2=] && fail 'got a zero exit code on invalid JSON'

		printf '\xff\xfe' | run >[2=] && fail 'got a zero exit code on invalid UTF-8'
	}
}

fn log {
	printf '%s ' +
	printf '%s ' $*
	echo
	$*
}

# TODO: utilize dynamic binding to its full extent.

actions = (
	run-cargo-test 'Run cargo test.' {
		let ((args rest) = <= { split-by-double-dash $* }) {
			cargo test <=$args -- --test-threads 1 <=$rest
		}
	}

	run-shell-tests 'Run shell tests.' {
		local (fn-run = @ { cargo run --quiet $* }) run-shell-tests
	}

	run-all-tests 'Run all tests. Arguments are ignored.' {
		$0 run-cargo-test || exit 1
		$0 run-shell-tests || exit 1
	}

	# TODO: this should be cleaned up and figured out.
	run-all-tests-with-coverage 'Runs all tests and collects test coverage information.' {
		log cargo llvm-cov clean --workspace
		log cargo llvm-cov --no-report --quiet -- --test-threads 1
		
		local (
			fn-run = @ {
				# Note: --ignore-run-fail isn't done, since we want to check the exit codes.
				log cargo llvm-cov run --no-report --quiet $*
			}
		) run-shell-tests

		log cargo llvm-cov report --ignore-filename-regex rust/library/std $*
	}

	test-with-coverage 'Run cargo llvm-cov.' {
		let ((args rest) = <= { split-by-double-dash $* }) {
			cargo llvm-cov <=$args --no-report --quiet -- --test-threads 1 <=$rest
		}
	}
)

# Set the default action to "help" if no arguments are given, otherwise it's "$1".
if { ~ $#* 0 } { action = help } { (action *) = $* }

for ((name _ function) = $actions) if { ~ $action $name } { $function $*; exit }

help

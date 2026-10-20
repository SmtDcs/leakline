let meta = #{
    name: "leakline-gate",
    description: "DeepSeek reviews one Leakline gate and runs its tests",
    when_to_use: "After Grok finishes a Leakline gate and before that gate is accepted",
    phases: [
        #{ title: "Review", detail: "DeepSeek checks the gate against the spec" },
        #{ title: "Test", detail: "DeepSeek runs the tests itself" },
    ],
};

let gate = if args == () { () } else { args.gate };
if gate == () {
    pause("verification", "Pass args.gate: spec, parse, policy, pczt, or readme.");
}

phase("Review");
let review_prompt = "You are the DeepSeek reviewer on the Leakline Zcash hackathon project at /home/samet/hackathons/zecathon-2026. ";
review_prompt += "The orchestrator is Grok. You do not change the spec, you do not edit files, and you do not add a network call, a server, a wallet, or a key. ";
review_prompt += "Read hackathon/10-pair-system.md and hackathon/01-plan.md with read_file before you answer. ";
review_prompt += "Gate name: " + gate + ". ";
review_prompt += "If the gate is spec, attack the decision table: missing leak, false CLEAN, policy lattice, raw-tx amount invention, unrecognized version. ";
review_prompt += "If the gate is parse, policy, pczt, or readme, read the Rust under that repo and cite file paths. ";
review_prompt += "If the gate is readme, also read README.md, examples/demo.sh, and SECURITY.md. ";
review_prompt += "A README claim the code does not implement is a finding. A non-claim listed in hackathon/10-pair-system.md and missing from the README is a finding. ";
review_prompt += "Do not ask for a server, a video upload, a wallet, or a key. ";
review_prompt += "A CLEAN verdict on a Sapling-Orchard valueBalance pair with no transparent parts is a defect. ";
review_prompt += "Reply with findings. Each finding names the file and the broken rule. An empty list is valid only after you have read the files.";
let review = agent(review_prompt, #{
    label: "deepseek-review",
    model: "deepseek-flash",
    capability_mode: "read-only",
});

phase("Test");
let test_prompt = "You are the DeepSeek tester on /home/samet/hackathons/zecathon-2026. Gate: " + gate + ". ";
test_prompt += "Do not edit source, do not change Cargo.toml, and do not print secrets. ";
test_prompt += "If the gate is spec, there is no binary yet: read hackathon/10-pair-system.md and say which three fixtures the table requires. Do not invent a test run. ";
test_prompt += "For every other gate, run `cargo test --offline` in that repo with the shell. If the crate is not downloaded yet, run `cargo test` once. ";
test_prompt += "Report the command, the exit code, and the failing test names. Do not claim a pass you did not see in the command output.";
let tested = agent(test_prompt, #{
    label: "deepseek-test",
    model: "deepseek-flash",
    capability_mode: "execute",
});

let review_ok = review != () && review.success;
let test_ok = tested != () && tested.success;
let summary = "gate " + gate;
if review_ok {
    summary += " review=ok";
} else {
    summary += " review=missing";
}
if test_ok {
    summary += " test=ok";
} else {
    summary += " test=missing";
}
complete(#{
    gate: gate,
    summary: summary,
    review_ok: review_ok,
    test_ok: test_ok,
});

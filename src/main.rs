/*  

    Ejemplo inspirado de https://github.com/lambdaclass/starknet_stack_prover_lambdaworks/blob/main/docs/src/starks/api.md

    Se hara una secuencia de Fibonacci de 32 elementos como traza de ejecucion y luego se verifica  
*/
use lambdaworks_math::field::fields::{
    fft_friendly::stark_252_prime_field::Stark252PrimeField as F,
    u64_prime_field::{F17, FE17},
};
use lambdaworks_stark::{
    starks::{
        example::{
            dummy_air::{self, DummyAIR},
            fibonacci_2_columns::{self, Fibonacci2ColsAIR},
            fibonacci_rap::{fibonacci_rap_trace, FibonacciRAP, FibonacciRAPPublicInputs},
            quadratic_air::{self, QuadraticAIR, QuadraticPublicInputs},
            simple_fibonacci::{self, FibonacciAIR, FibonacciPublicInputs},
        },
        proof::options::{ProofOptions, SecurityLevel},
        prover::prove,
        trace::TraceTable,
        verifier::verify,
    },
    FE,
};

fn main(){

    let trace_length = 32;

    let trace = simple_fibonacci::fibonacci_trace([FE::from(1), FE::from(1)], trace_length);
    let proof_options = ProofOptions::default_test_options();

    let pub_inputs = FibonacciPublicInputs {
        a0: FE::one(),
        a1: FE::one(),
    };

    let proof = prove::<F, FibonacciAIR<F>>(&trace, &pub_inputs, &proof_options).unwrap();
    assert!(verify::<F, FibonacciAIR<F>>(&proof, &pub_inputs, &proof_options));
}
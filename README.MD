# Cabidge

## The insides

### Representation stages
Frontend Compiler:
- Source code
- IR (may be multiple stages)
- cabidge-code (to file)

VM:
- cabidge-code (from file)
- Annotated CPS
    - Like cabidge-code but with annotated types and more operations,
    - Standard library functions are written in this (meaning most operators are not directly exposed to frontends)
- Strong SSA
    - Only necessary if I can't easily reuse memory in previous stages
- Cranelift
- Machine code?

## First Steps
- [x] Minimal module format
- [ ] Minimal runtime
    - Programs return int32
- [ ] Minimal language frontend compiler
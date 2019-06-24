# observer
Observability for rust servers


# Today

- [x] verify it checks/runs
- [x] document the output somewhere
- [x] see how output looks when observed functions are nested


- [ ] `observed` should use function name by default
- [ ] result handling: if result of an observed function implements Resulty,
      then call resulty.to_string(), else call Display, if none, code cant 
      compile

# Later

- [ ] how to make function names unique?
- [ ] integrate with newrelic stuff
- [ ] Queue trait to be stored on context object
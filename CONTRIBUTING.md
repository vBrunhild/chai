# Contributing Guide
Thanks a lot for considering contributing to this project!
Here are some things to be aware before starting.

---

# Development Workflow
 1. Fork this repository.
 2. Clone your fork locally.
 ```bash
 git clone https://github.com/<your-username>/<your-repo>.git
 cd <your-repo>
 ````
 3. Create a branch:
 ```bash
 git checkout -b <feature/change>
 ```
 4. Make your changes, run tests then commit them while following our [Commit Guidelines](CONTRIBUTING.md#Commit Guidelines).
 5. Push branch to your fork.
 ```
 git push 
 ```
 6. Open a pull request from your fork to the main repository.

---

# Commit Guidelines
Please follow [Conventional Commits](https://www.conventionalcommits.org/), here's a quick read through:
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Types
Use the most fitting type:
| Type | When to Use |
| -------------- | --------------- |
| feat | New feature (visible to user) |
| fix  | Bug fix (corrects behavior) |
| docs | Documentation changes only |
| style | Code style / formatting (no logic or behavioral changes) |
| refactor | Code restructuring without behavioral changes |
| test | Adding or modifying test |
| build | Build system or dependencies change |
| chore | Boring stuff like tooling or linting, configuration |

## Examples
* `feat(cli): create command`
* `fix(openAi): stop responses from hanging`
* `docs(readme): add contributing guide`

---

Thanks for helping making this project the best it can be!

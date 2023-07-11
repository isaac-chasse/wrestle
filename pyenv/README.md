# Python Environment Managers and their Access Patterns 

Python's best-worst feature is the lack of an officially supported dependency manager. This comes can allow for developers to customize freely, but comes at the cost of developer time when working with teams and different machines. Part of the goal with `wrestle` is to better define these managers and their access patterns so we can streamline this inter-developer/machine headache. 

## Some of the problems solved by dependency managers
1. Project specific dependency definitions. One project might require `django==2.6` while another requires `django==3.0.6`
2. Simplified package and library management. 
3. Dependency resolution. Look into the project's sub-dependencies and reolves conflicts.
4. Reproduction of environments. One machine can handle many different project requirements and environments.

## Some of the problems created by Python's dependency managers.
Python is a complex build system that relies on C, C++, Cython, etc. just to build environments and dependencies. This leads to a non-zero amount of overhead when transferring these environment specifications between team members or machines. "Well it runs on my machine" is something we've all heard before. 

## What are we trying to `wrestle`?
The goal with `wrestle` is to put the environment management into the hands of the developer, allowing them to use whatever dependecy manager they want, and simplify the process of creating and maintaining dependency manifests. We have all worked with someone who sends you their `requirements.txt` and it looks like this:

```
pandas 
numpy 
tensorflow
pytorch
some-package-youve-never-heard-of
```

The goal with wrestle is to allow you and your teammates to coordinate a "ground truth" dependency manifest in whatever markup file (.txt, .yml, .toml, etc.) works for your team, and translate that manifest into whatever markup file works for *you*.

If you work with folks who are hardcore about `conda` but you like `poetry`, the goal is that you can simply `wrestle --source=conda --target=poetry` to create a `pyproject.toml` from the `environment.yaml` your teammates use. 

# Python Dependency Managers

## Venv
The default virtual environment module for Python 3. Uses `requirements.txt`.

## Virtualenv
Most common virtual environment module for Python 2. Uses `requirements.txt`.

## Pipenv


## Conda

## Poetry


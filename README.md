# git-bisect-demo

Example repo to demonstrate git bisect usage

Contains the best code you'll ever see

## Usage
Start bisecting with:
```
> git bisect start <bad commmit> <good commit(s)>
```

Example:
```
> git bisect start HEAD HEAD~50
```

Then run the tests and mark each commit bad/good:
```
> cargo test
> git bisect bad/good
```

Alternatively, let git do everything:
```
> git bisect run cargo test
```

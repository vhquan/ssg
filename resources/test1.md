---
title: How to program with Rust
date: 2025-03-19
author: Jane Smith
tags: [rust, programming, fitness]
summary: The Rust programming language
---

## Getting Started with Rust

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

## Why Rust?

Rust is fundamentally about empowerment: no matter what kind of code you are writing now, Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before.

This is JSON
```json
{
  "firstName": "John",
  "lastName": "Smith",
  "age": 25
}
```

This is C++
```C++
#include <bits/stdc++.h>

template <typename T>
int binSearch(std::vector<T>& arr, T target) {
    int low = 0, high = arr.size() - 1;

    while (low <= high) {
        int mid = low + (high - low)/2;

        if (arr[mid] == target) return mid;
        else if (arr[mid] > target) high = mid - 1;
        else low = mid + 1;
    }

    return -1;
}
```
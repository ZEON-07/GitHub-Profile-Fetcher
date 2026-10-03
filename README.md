# CLI GitHub Profile Fetcher (Starter Code)

A simple command-line interface (CLI) tool written in Go that fetches public profile data for a GitHub user using the GitHub REST API.

Currently, this starter version retrieves the profile data using `http.Get` and lazily dumps the raw, unformatted JSON payload straight into the terminal.

---

## Getting Started

### Prerequisites

- [Go](https://go.dev/doc/install) (version 1.18 or higher recommended)

### Running the Application

To run the program, pass any GitHub username as a command-line argument:

```bash
go run main.go <username>
```

#### Example:

```bash
go run main.go octocat
```

---

## Tasks / Issues to Fix

Participants should update the code to resolve the following issues:

1. **Create a Go struct to properly parse the JSON payload**
   - Define a custom `struct` with matching JSON tags (`json:"..."`) using `encoding/json`.
   - Unmarshal the API response body into an instance of your struct instead of treating it as raw text.

2. **Format the terminal output cleanly**
   - Instead of printing raw JSON, display a structured and readable summary showing only:
     - **Name**
     - **Bio**
     - **Public Repos** (count)

3. **Add error handling for missing users (404 response)**
   - Check the HTTP status code (`resp.StatusCode`).
   - If the API returns `404 Not Found`, intercept it and print `"User not found"` cleanly instead of dumping an error payload or crashing.

package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
)

type GitHubUser struct {
	Name        string `json:"name"`
	Bio         string `json:"bio"`
	PublicRepos int    `json:"public_repos"`
}

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Usage: go run main.go <username> [--gitlab]")
		return
	}

	gitlab := false
	if len(os.Args) > 2 && os.Args[2] == "--gitlab" {
		gitlab = true
	}
	username := os.Args[1]
	if gitlab {
		fetchGitLabUser(username)
	} else {
		fetchGitHubUser(username)
	}
}

func fetchGitLabUser(username string) {
	url := fmt.Sprintf("https://gitlab.com/api/v4/users?username=%s", username)

	resp, err := http.Get(url)
	if err != nil {
		fmt.Println("Error fetching data:", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusNotFound {
		fmt.Println("User not found")
		return
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("Error reading response:", err)
		return
	}

	var users []struct {
		Name        string `json:"name"`
		Bio         string `json:"bio"`
		PublicRepos int    `json:"public_repos"`
	}
	if err := json.Unmarshal(body, &users); err != nil {
		fmt.Println("Error parsing response:", err)
		return
	}

	if len(users) == 0 {
		fmt.Println("User not found")
		return
	}

	user := users[0]
	fmt.Println("Name:", user.Name)
	fmt.Println("Bio:", user.Bio)
	fmt.Println("Public Repos:", user.PublicRepos)
}

func fetchGitHubUser(username string) {
	url := fmt.Sprintf("https://api.github.com/users/%s", username)

	resp, err := http.Get(url)
	if err != nil {
		fmt.Println("Error fetching data:", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusNotFound {
		fmt.Println("User not found")
		return
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("Error reading response:", err)
		return
	}

	var user GitHubUser
	if err := json.Unmarshal(body, &user); err != nil {
		fmt.Println("Error parsing response:", err)
		return
	}

	fmt.Println("Name:", user.Name)
	fmt.Println("Bio:", user.Bio)
	fmt.Println("Public Repos:", user.PublicRepos)
}

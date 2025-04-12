# Your Money Left The Chat

## 📀 Demo Video

[![Demo Video](./screenshots/demo.png)](https://youtu.be/1X9WQuZ4N5w)

## TL;DR

A Rust-native 🦀, SQLite-backed, LLM-powered financial tracking system

AI-powered money tracker that logs your spending, analyzes your habits, and calculates taxes. all through natural conversation.

Talk to Claude or Ollama, and the system will store everything locally using SQLite, no cloud bullshit.

## 🧠 Features

- ✍️ **Natural Language Input**  
  Log expenses like:

  > “Spent 129 baht on KFC today” → Stored instantly

- 📆 **Smart Summaries**  
  Daily, monthly, yearly breakdowns, including category-based analytics

- 📊 **Graph View**  
  Visualize how much of your soul goes into food every month

- 📉 **Tax Calculation**  
  Estimate how much you’ll owe Uncle Sam (or whoever)

- 💀 **Debt Management**  
  Track who owes you money and who’s ghosting you

- 🔒 **Offline & Local-First**  
  100% yours – no internet, no accounts, no surveillance capitalism

## 🦀 Tech Stack

| Layer        | Tech                            |
| ------------ | ------------------------------- |
| Backend      | MCP Rust SDK + SQLite + Diesel  |
| AI Interface | Claude / Ollama (via `mcp-cli`) |

## 🔧 Getting Started

1. **Install Rust, SQLite, and Makefile**

   Rust: https://www.rust-lang.org/tools/install  
   SQLite: https://sqlite.org/download.html  
   GNU Make: https://www.gnu.org/software/make/

2. **Clone the Repo**

   ```bash
   https://github.com/Rayato159/your-money-left-the-chat
   cd your-money-left-the-chat
   ```

3. **Install Diesel CLI**

   ```bash
    cargo install diesel_cli --no-default-features --features sqlite
   ```

   > If you facing with the error that can't find `sqlite3.lib`, Please install `sqlite3` on your machine first.

4. **Create Sqlite Database**

   Just create file `database.db` in the:
   `./src/infrastructure/database/sqlite_data/`

5. **Database Setup**

   Makefile:

   ```bash
   make migrate-up
   ```

   Or using cargo as original

   ```bash
   diesel migrate run
   ```

6. **Build the Project**

   Makefile:

   ```bash
    make build-release
   ```

   Or using cargo as original

   ```bash
   cargo build --release --example your_money_left_the_chat
   ```

7. **Edit the config file**

   If you using Claude Desktop just find the `claude_desktop_config.json`, then add this

   Windows:

   ```json
   {
     "mcpServers": {
       "money": {
         "command": "PATH-TO/your-money-left-the-chat/target/release/examples/your_money_left_the_chat.exe",
         "args": [
           "PATH-TO/your-money-left-the-chat/src/infrastructure/database/sqlite_data/database.db"
         ]
       }
     }
   }
   ```

   Linux/MacOS:

   ```json
   {
     "mcpServers": {
       "money": {
         "command": "PATH-TO/your-money-left-the-chat/target/release/examples/your_money_left_the_chat",
         "args": [
           "PATH-TO/your-money-left-the-chat/src/infrastructure/database/sqlite_data/database.db"
         ]
       }
     }
   }
   ```

8. **Chatting with your dude (Claude):**

   ```text
    You: "Hey, I spent 129 baht on KFC today"
    Claude: "Got it! I've logged that expense for you. Anything else?"
   ```

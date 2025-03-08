# Log Inspector

AI-powered log analysis tool that uses OpenAI to classify errors and provide intelligent summaries of your log files.

## Features
- Intelligent log chunking for optimal analysis
- Error classification with priority ranking
- Detailed summaries with metrics
- Multiple configuration options for flexibility

## Installation

Install via Homebrew:
```bash
$ brew tap hoaihuongbk/log-inspector https://github.com/hoaihuongbk/log-inspector
$ brew install log-inspector
```

## Configuration
Create a configuration file at ~/.log-inspector.cnf:
```bash
OPENAI_API_KEY=your-api-key-here
OPENAI_HOST=https://api.openai.com
```

Configuration priority:
1. System environment variables
2. User config file (~/.log-inspector.cnf)
3. Local .env file

## Usage
Analyze a log file:

```bash
$ log-inspector path/to/your/logfile.log
```

The tool will:

- Classify errors by type and priority
- Provide detailed summaries with metrics
- Handle large files through intelligent chunking

## Output Example

```bash
Error Codes: SPARK_OOM_ERROR, NETWORK_ERROR
Summary: Application experienced memory issues during peak load.
- Memory usage peaked at 85% (13.6GB/16GB)
- Network latency increased to 2500ms
- Database connections failed after 3 retries
```

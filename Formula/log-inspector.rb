# Updated by GitHub Actions on 2025-03-08 12:50:58
class LogInspector < Formula
  desc "AI-powered log analysis tool"
  homepage "https://github.com/hoaihuongbk/log-inspector"
  version "0.1.2"

  url "https://github.com/hoaihuongbk/log-inspector/releases/download/v#{version}/log-inspector"
  sha256 "32ac858104d3cd784b6675fd0407985c69f0b559c70fd661449691e757ddc73e"

  def install
    bin.install "log-inspector"
  end

  test do
    system "#{bin}/log-inspector", "--version"
  end
end

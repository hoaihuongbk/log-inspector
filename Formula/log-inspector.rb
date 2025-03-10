# Updated by GitHub Actions on 2025-03-10 15:59:22
# Updated by GitHub Actions on 2025-03-08 12:50:58
class LogInspector < Formula
  desc "AI-powered log analysis tool"
  homepage "https://github.com/hoaihuongbk/log-inspector"
  version "0.1.3"

  url "https://github.com/hoaihuongbk/log-inspector/releases/download/v#{version}/log-inspector"
  sha256 "b75e82e298c8e948532b85dcfa4fce6fca8cf120daca099b426fec59d9cee0b6"

  def install
    bin.install "log-inspector"
  end

  test do
    system "#{bin}/log-inspector", "--version"
  end
end

defmodule ElixirRustInteropDemo.Mixfile do
  use Mix.Project

  def project do
    [app: :elixir_rust_interop_demo,
     version: "0.0.1",
     elixir: "~> 1.2",
     compilers: [:cargo, :elixir, :app],
     build_embedded: Mix.env == :prod,
     start_permanent: Mix.env == :prod,
     deps: deps]
  end

  # Configuration for the OTP application
  #
  # Type "mix help compile.app" for more information
  def application do
    [applications: [:logger]]
  end

  defp deps do
    []
  end
end


####################
# Rust Cargo Tasks #
####################

defmodule Mix.Tasks.Compile.Cargo do
  @shortdoc "Compiles helper in rust_src"

  def run(_) do
    case System.cmd("cargo", ["build", "--release"],
                    cd: "rust_src",
                    stderr_to_stdout: true) do
      {result, 0} ->
        if result != "" do
          Mix.shell.info result
        end
        # @TODO: Skip coping the file if it is up-to-date.
        case System.cmd("cp", ["-p", "rust_src/target/release/libpi_nif.so", "priv"],
                        stderr_to_stdout: true) do
          {"", 0} ->
            :ok
          {result, 0} ->
            Mix.shell.info result
            :ok
          {result, _error_code} ->
            Mix.shell.error result
            raise "copying libpi_nif.so failed"
        end
      {result, _error_code} ->
        Mix.shell.error result
        raise "cargo build --release failed."
    end
  end
end

defmodule Mix.Tasks.Clean.Cargo do
  @shortdoc "Cleans helper in rust_src"

  def run(_) do
    case System.cmd("cargo ", ["clean"],
                    cd: "rust_src",
                    stderr_to_stdout: true) do
      {result, 0} ->
        Mix.shell.info result
        :ok
      {result, _error_code} ->
        Mix.shell.error result
        :ok
    end
  end
end

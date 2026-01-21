
defmodule ML do

  def main() do

    dbg("ML.main")

    repo = {:hf, "sentence-transformers/all-MiniLM-L6-v2"}
    {:ok, model_info} = Bumblebee.load_model(repo)
    {:ok, tokenizer} = Bumblebee.load_tokenizer(repo)

    # 2. Build the serving
    # Note: output_pool: :mean_pooling is standard for sentence-transformers models
    serving = Bumblebee.Text.text_embedding(model_info, tokenizer,
    output_pool: :mean_pooling,
    output_attribute: :hidden_state)

    # 3. Run the serving on your text
    text = "Bumblebee makes Elixir AI easy."
    result = Nx.Serving.run(serving, text)

    # Access the embedding tensor
    IO.inspect(result.embedding)


  end

end

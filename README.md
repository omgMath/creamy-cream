# Creamy Cream

## Idea
According to Harold McGee's On Food and Cooking, ice cream by weight typically targets approximately:
- 60% water
- 20% fat
- 15–20% sugar.

This project computes how much of each ingredient to use to reach a that "perfect ratio". It only considers fat, sugar and water. Other things influencing the ice cream (pectins, salt, etc) are not considered.

**Example**
Let's make 100g ice cream that consists of almonds, sugar and water. To achieve that ratio, I should use 
- 31.2g almonds
- 12.2g sugar
- 56.6g water.

## Contributing

### Dev setup
- Use the configured devcontainer
- `dx serve` - to serve the web app
- `npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch` - to continuosly compile the tailwind `input.css`

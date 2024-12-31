import { html, render } from "lit-html";
import { piwallet_backend } from "declarations/piwallet_backend";
import logo from "./logo2.svg";

class WalletApp {
	balance = 0;
	totalSupply = 0;
	addressStatus = "";
	feedback = "";

	constructor() {
		this.#render();
	}

	#getBalance = async () => {
		this.balance = await piwallet_backend.get_balance();
		this.feedback = `Your balance is: ${this.balance} tokens`;
		this.#render();
	};

	#getTotalSupply = async () => {
		this.totalSupply = await piwallet_backend.get_total_supply();
		this.feedback = `Total token supply: ${this.totalSupply}`;
		this.#render();
	};

	#checkAddress = async (e) => {
		e.preventDefault();
		const address = document.getElementById("address").value;
		const exists = await piwallet_backend.address_exists(address);
		this.addressStatus = exists
			? "Address exists in the system."
			: "Address does not exist.";
		this.#render();
	};

	#sendTokens = async (e) => {
		e.preventDefault();
		const to = document.getElementById("sendTo").value;
		const amount = Number(document.getElementById("sendAmount").value);
		const result = await piwallet_backend.send_tokens(to, amount);
		this.feedback = `Send Tokens Result: ${JSON.stringify(result)}`;
		this.#render();
	};

	#render() {
		const body = html`
			<style>
				main {
					font-family: Arial, sans-serif;
					max-width: 600px;
					margin: 0 auto;
					padding: 20px;
					background: #f9f9f9;
					border-radius: 8px;
					box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
				}

				img {
					max-width: 100px;
					display: block;
					margin: 0 auto 20px;
				}

				h1 {
					text-align: center;
					color: #333;
				}

				section {
					margin-bottom: 20px;
				}

				button {
					background: #007bff;
					color: #fff;
					border: none;
					padding: 10px 15px;
					border-radius: 4px;
					cursor: pointer;
					font-size: 14px;
				}

				button:hover {
					background: #0056b3;
				}

				input {
					width: calc(100% - 20px);
					padding: 8px 10px;
					margin-bottom: 10px;
					border: 1px solid #ddd;
					border-radius: 4px;
					font-size: 14px;
				}

				form {
					display: flex;
					flex-direction: column;
				}

				p {
					color: #555;
					font-size: 14px;
					text-align: center;
				}

				@media (max-width: 480px) {
					main {
						padding: 10px;
					}

					button {
						font-size: 12px;
						padding: 8px 10px;
					}

					input {
						font-size: 12px;
					}
				}
			</style>

			<main>
				<img src="${logo}" alt="Logo" />
				<h1>Wallet Application</h1>

				<section>
					<button @click="${this.#getBalance}">Check Balance</button>
					<button @click="${this.#getTotalSupply}">
						Get Total Supply
					</button>
				</section>

				<section>
					<form @submit="${this.#checkAddress}">
						<input
							id="address"
							type="text"
							placeholder="Enter Address"
						/>
						<button type="submit">Check Address</button>
					</form>
					<p>${this.addressStatus}</p>
				</section>

				<section>
					<form @submit="${this.#sendTokens}">
						<input
							id="sendTo"
							type="text"
							placeholder="Recipient Address"
						/>
						<input
							id="sendAmount"
							type="number"
							placeholder="Amount to Send"
						/>
						<button type="submit">Send Tokens</button>
					</form>
				</section>

				<section>
					<p>${this.feedback}</p>
				</section>
			</main>
		`;

		render(body, document.getElementById("root"));
	}
}

export default WalletApp;

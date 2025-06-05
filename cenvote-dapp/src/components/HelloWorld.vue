<template>
  <div class="container">
    

    <center>
      <h1 style="font-family: cursive;">CenVote</h1>
      <h2 class="py-3">Sistema de Votación Descentralizada</h2>
      <button class="btn " style="padding: 10px; width: 150px; height: 50px;background-color: #1a5276; border-radius: 30px; color: white;"
        @click="conectarWallet">
        Conectar Wallet
      </button>
      <p :style="{ color: walletStatusColor }">{{ walletStatus }}</p>
    </center>

    <!-- Formulario para registrar usuario -->
    <div class="container">
      <form @submit.prevent="registrarUsuario" class="my-4 p-3 rounded shadow bg-primary text-black">
        <center>
          <h2 class="rounded-3">Registrar Usuario</h2>
        </center>
        <input v-model="user.user_id" placeholder="ID de usuario" required />
        <input v-model="user.first_name" placeholder="Nombre" required />
        <input v-model="user.paternal_last_name" placeholder="Apellido paterno" required />
        <input v-model="user.maternal_last_name" placeholder="Apellido materno" required />
        <input v-model="user.phone" placeholder="Teléfono" required />
        <input v-model="user.email" placeholder="Correo electrónico" required />
        <div class="d-flex justify-content-center gap-5 mt-2">
          <button type="submit" class="btn btn-warning" style="height: 40px; width: 150px; border-radius: 30px;">Registrar Usuario</button>
          <button type="button" class="btn btn-warning" style="height: 40px; width: 150px;border-radius: 30px; ">Cancelar</button>
        </div>
      </form>
    </div>

    <!-- Formulario para registrar candidato -->
    <div class="container">
      <form @submit.prevent="registrarCandidato" class="my-4 p-3 rounded shadow bg-primary text-black">
        <center>
          <h2>Registrar Candidato</h2>
        </center>
        <input v-model="candidate.candidate_id" placeholder="ID del candidato" required />
        <input v-model="candidate.user_id" placeholder="ID del usuario" required />
        <input v-model="candidate.rfc" placeholder="RFC" required />
        <div class="d-flex justify-content-center gap-5 mt-2" >
          <button type="submit" class="btn btn-warning" style="height: 40px; width: 160px;border-radius: 30px; ">Registrar Candidato</button>
          <button type="button" class="btn btn-warning" style="height: 40px; width: 150px;border-radius: 30px;  ">Cancelar</button>
        </div>
      </form>
    </div>
    <!-- Formulario para emitir un voto -->
    <div class="container">
      <form @submit.prevent="emitirVoto" class="my-4 p-3 rounded shadow bg-primary text-black">
        <center>
          <h2>Votar por Candidato</h2>
        </center>
        <input v-model="vote.id_voto" placeholder="ID del voto" required />
        <input v-model="vote.id_usuario" placeholder="ID del usuario" required />
        <input v-model="vote.id_candidato" placeholder="ID del candidato" required />
        <input v-model="vote.fecha_voto" type="date" required />
        <div class="d-flex justify-content-center gap-5 mt-2">
          <button type="submit" class="btn btn-warning" style="height: 40px; width: 150px;border-radius: 30px; ">Votar</button>
          <button type="button" class="btn btn-warning" style="height: 40px; width: 150px;border-radius: 30px;  ">Cancelar</button>
        </div>
      </form>
    </div>
    <!-- Formulario para crear encuesta -->
    <div class="container">
      <form @submit.prevent="crearEncuesta" class="my-4 p-3 rounded shadow bg-primary text-black">
        <center>
          <h2>Crear Encuesta</h2>
        </center>
        <input v-model="survey.id_encuesta" placeholder="ID de encuesta" required />
        <input v-model="survey.nombre_encuesta" placeholder="Nombre de encuesta" required />
        <input v-model="survey.descripcion" placeholder="Descripción" required />
        <input v-model="survey.fecha_creacion" type="date" required />
        <input v-model="survey.fecha_culminacion" type="date" required />
        <input v-model="survey.id_voto" placeholder="ID de voto" required />
        <input v-model="survey.id_candidato" placeholder="ID del candidato" required />
        <div class="d-flex justify-content-center gap-5 mt-2">
          <button type="submit" class="btn btn-warning" style="height: 40px; width: 150px;border-radius: 30px; ">Crear Encuesta</button>
          <button type="button" class="btn btn-warning" style="height: 40px; width: 150px;border-radius: 30px; ">Cancelar</button>
        </div>
      </form>
    </div>

  </div>

</template>

<script>
export default {
  data() {
    return {
      CONTRACT_ID: "GBART5Q66ONK3R2CC5NOXPVHU2R26ONWNYQHVTQM3FWTT3WKPFAE53BZ",
      publicKey: null,
      walletStatus: "",
      walletStatusColor: "black",
      user: {
        user_id: "",
        first_name: "",
        paternal_last_name: "",
        maternal_last_name: "",
        phone: "",
        email: "",
      },
      candidate: {
        candidate_id: "",
        user_id: "",
        rfc: "",
      },
      vote: {
        id_voto: "",
        id_usuario: "",
        id_candidato: "",
        fecha_voto: "",
      },
      survey: {
        id_encuesta: "",
        nombre_encuesta: "",
        descripcion: "",
        fecha_creacion: "",
        fecha_culminacion: "",
        id_voto: "",
        id_candidato: "",
      },
      auth: {
        id: "",
        rfc: "",
      },
    };
  },
  methods: {
    async conectarWallet() {
      try {
        const freighter = window.freighterApi || window.freighter || (window.self !== window.top && window.parent.freighter);
        if (!freighter) {
          throw new Error("Freighter no está instalado o no está disponible en esta página.");
        }
        this.publicKey = await freighter.getPublicKey();
        const network = await freighter.getNetwork();
        if (network !== "TESTNET") {
          alert("Estás en la red equivocada. Por favor, cambia a TESTNET desde Freighter.");
          return;
        }
        this.walletStatus = `Wallet conectada: ${this.publicKey}`;
        this.walletStatusColor = "green";
        alert(`Wallet conectada: ${this.publicKey}\nRed: ${network}`);
      } catch (error) {
        console.error("Error al conectar con Freighter:", error);
        this.walletStatus = "Error al conectar wallet";
        this.walletStatusColor = "red";
        if (error.message.includes("Freighter")) {
          window.open("https://www.freighter.app", "_blank");
        }
      }
    },
    async invokeContractFunction(method, args = {}) {
      if (!this.publicKey) {
        const shouldConnect = confirm("Debes conectar tu wallet primero. ¿Conectar ahora?");
        if (shouldConnect) {
          await this.conectarWallet();
        } else {
          return;
        }
      }
      try {
        const freighter = window.freighterApi || window.freighter;
        // Construir el transaction XDR (suponiendo método y args)
        const transactionXDR = await freighter.getTransaction({
          network: "TESTNET",
          source: this.publicKey,
          contractId: this.CONTRACT_ID,
          method,
          args,
        });
        const txResponse = await freighter.signAndSubmitTransaction(transactionXDR);
        alert(`Transacción enviada\nHash: ${txResponse.hash}`);
        console.log("Transacción completada:", txResponse);
        return txResponse;
      } catch (err) {
        console.error("Error en invokeContractFunction:", err);
        alert(`Error en la transacción:\n${err.message}`);
        throw err;
      }
    },
    async registrarUsuario() {
      try {
        await this.invokeContractFunction("registrar_usuario", this.user);
        // Limpiar formulario tras registro
        this.user = {
          user_id: "",
          first_name: "",
          paternal_last_name: "",
          maternal_last_name: "",
          phone: "",
          email: "",
        };
      } catch (error) {
        console.error(error);
      }
    },
    async registrarCandidato() {
      try {
        await this.invokeContractFunction("registrar_candidato", this.candidate);
        this.candidate = { candidate_id: "", user_id: "", rfc: "" };
      } catch (error) {
        console.error(error);
      }
    },
    async emitirVoto() {
      try {
        await this.invokeContractFunction("votar", this.vote);
        this.vote = { id_voto: "", id_usuario: "", id_candidato: "", fecha_voto: "" };
      } catch (error) {
        console.error(error);
      }
    },
    async crearEncuesta() {
      try {
        await this.invokeContractFunction("crear_encuesta", this.survey);
        this.survey = {
          id_encuesta: "",
          nombre_encuesta: "",
          descripcion: "",
          fecha_creacion: "",
          fecha_culminacion: "",
          id_voto: "",
          id_candidato: "",
        };
      } catch (error) {
        console.error(error);
      }
    },
    async autenticar() {
      try {
        await this.invokeContractFunction("autenticar", this.auth);
        this.auth = { id: "", rfc: "" };
      } catch (error) {
        console.error(error);
      }
    },
  },
};
</script>

<!-- Puedes agregar Bootstrap desde CDN en tu index.html o aquí si quieres -->
<style scoped>
input,
button {
  display: block;
  margin: 0.5rem 0;
  padding: 0.5rem;
  width: 100%;
  box-sizing: border-box;
}

</style>

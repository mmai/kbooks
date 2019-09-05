<template>
  <v-container>
    <v-layout text-center wrap>
      <v-flex xs12>
        <v-img
          :src="require('../assets/logo.svg')"
          class="my-3"
          contain
          height="200"
        ></v-img>
      </v-flex>

      <v-flex mb-4>
        <h1 class="display-2 font-weight-bold mb-3" v-translate>
          Welcome to Kbooks
        </h1>
        <div>
          <select name="language" v-model="$language.current">
            <option
              v-bind:key="key"
              v-for="(language, key) in $language.available"
              :value="key"
              >{{ language }}</option
            >
          </select>
        </div>
        <h2>
          {{ message }}
        </h2>
      </v-flex>
    </v-layout>
  </v-container>
</template>

<script>
export default {
  name: "home",
  props: {
    action: String
  },
  computed: {
    message() {
      switch (this.action) {
        case "registerOk":
          return "Your acount has been created, you can now log in with your username and password.";
        default:
          return "";
      }
    }
  }
};
</script>

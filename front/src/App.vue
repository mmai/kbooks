<template>
  <v-app>
    <v-app-bar app>
      <v-toolbar-title class="headline text-uppercase">
        <span>Kbooks</span>
        <span class="font-weight-light"> An Actix + Vue experiment</span>
      </v-toolbar-title>
      <v-spacer></v-spacer>
      <span class="mr-2">
        <auth></auth>
      </span>
      <v-btn
        text
        href="https://github.com/mmai/khnum/releases/latest"
        target="_blank"
      >
      </v-btn>
    </v-app-bar>

    <v-content>
      <div id="nav"><router-link to="/">Home</router-link> |</div>
      <router-view />
    </v-content>
  </v-app>
</template>

<script>
// @ is an alias to /src
import Auth from "@/components/Auth.vue";

export default {
  name: "App",
  components: {
    Auth
  },
  data: () => ({
    //
  })
};
</script>

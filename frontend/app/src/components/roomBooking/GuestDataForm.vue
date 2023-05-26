<template>
  <v-container>
    <v-form ref="form">
      <v-row>
        <v-col>
          <v-text-field
            v-model="firstName"
            label="First name"
            variant="underlined"
            :rules="[rules.required, rules.lessThanThirty, rules.moreThanTwo]"
            prepend-icon="mdi-account"
            counter="30"
            required
            validate-on="input"
            @input="updateData"
          >
          </v-text-field>
        </v-col>
        <v-col>
          <v-text-field
            v-model="lastName"
            label="Last name"
            variant="underlined"
            :rules="[rules.required, rules.lessThanThirty, rules.moreThanTwo]"
            prepend-icon="mdi-account"
            counter="30"
            required
            validate-on="input"
            @input="updateData"
          >
          </v-text-field>
        </v-col>
      </v-row>
      <v-row>
        <v-col>
          <v-text-field
            v-model="email"
            label="E-Mail"
            variant="underlined"
            :rules="[rules.required, rules.isEmail]"
            prepend-icon="mdi-email"
            counter="30"
            required
            placeholder="example@mail.com"
            validate-on="input"
            @input="updateData"
          >
          </v-text-field>
        </v-col>
        <v-col>
          <v-text-field
            v-model="phoneNumber"
            label="Phone number"
            variant="underlined"
            :rules="[rules.required, rules.isPhoneNumber]"
            prepend-icon="mdi-phone"
            required
            placeholder="+4912345678"
            validate-on="input"
            hint="Phone number starts with a country code"
            @input="updateData"
          >
          </v-text-field>
        </v-col>
      </v-row>
      <v-row>
        <v-col>
          <v-text-field
            v-model="address"
            label="Address"
            variant="underlined"
            :rules="[rules.required, rules.moreThanTwo]"
            prepend-icon="mdi-home"
            counter="100"
            required
            validate-on="input"
            @input="updateData"
          >
          </v-text-field>
        </v-col>
        <v-col>
          <v-text-field
            v-model="passportNumber"
            label="Passport number"
            variant="underlined"
            :rules="[rules.required, rules.isPassportNumber]"
            prepend-icon="mdi-passport"
            required
            validate-on="input"
            @input="updateData"
            @keyup="uppercase"
          >
          </v-text-field>
        </v-col>
      </v-row>
    </v-form>
  </v-container>
</template>

<script>
import { nextTick } from 'vue';
export default {
  name: "guest-data-form",
  emits: ["input"],
  props: {
    guestNumber: {
      type: Number,
      required: true,
    },
    guestData: {
      type: Object,
      required: true,
    },
  },
  data() {
    return {
      firstName: "",
      lastName: "",
      email: "",
      address: "",
      phoneNumber: "",
      passportNumber: "",
      rules: {
        required: (value) => !!value || "Field is required",
        lessThanThirty: (value) =>
          (!!value && value.length <= 30) ||
          "Value must not be longer than 30 characters",
        moreThanTwo: (value) =>
          (!!value && value.length > 2) ||
          "Value must be longer than 2 characters",
        isEmail: (value) =>
          !value ||
          /^\w+([.-]?\w+)*@\w+([.-]?\w+)*(\.\w{2,3})+$/.test(value) ||
          "E-mail is invalid",
        isPhoneNumber: (value) =>
          !value ||
          /^\+?[1-9][0-9]{7,14}$/.test(value) ||
          "Phone number is invalid",
        isPassportNumber: (value) =>
          !value ||
          /^(?!^0+$)[a-zA-Z0-9]{3,20}$/.test(value) ||
          "Passport number is invalid",
      },
    };
  },
  computed: {
    getGuestData() {
      return {
        firstName: this.firstName,
        lastName: this.lastName,
        email: this.email,
        address: this.address,
        phoneNumber: this.phoneNumber,
        passportNumber: this.passportNumber,
      };
    },
  },
  methods: {
    async validate() {
      if (
        !(
          this.firstName &&
          this.lastName &&
          this.email &&
          this.address &&
          this.phoneNumber &&
          this.passportNumber
        )
      ) {
        return false;
      }
      const formValidation = await this.$refs.form.validate();
      return formValidation.valid;
    },
    async updateData() {
      await this.$emit("input", {
        guestNumber: this.guestNumber,
        guestData: this.getGuestData,
        valid: (await this.validate()) == true,
      });
    },
    uppercase() {
        this.passportNumber = this.passportNumber.toUpperCase()
    }
  },
  async created() {
    this.firstName = this.guestData?.firstName ?? this.firstName;
    this.lastName = this.guestData?.lastName ?? this.lastName;
    this.email = this.guestData?.email ?? this.email;
    this.address = this.guestData?.address ?? this.address;
    this.phoneNumber = this.guestData?.phoneNumber ?? this.phoneNumber;
    this.passportNumber = this.guestData?.passportNumber ?? this.passportNumber;
    await nextTick()
    this.updateData()
  },
};
</script>

<style>
</style>
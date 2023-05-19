import { Component } from '@angular/core';
import { FormGroup, FormBuilder } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthenticationService } from '../authentication.service';
import { Otp } from '../otp';
import { InfoLogin } from '../info-login';

@Component({
  selector: 'app-user-send-otp',
  templateUrl: './user-send-otp.component.html',
  styleUrls: ['./user-send-otp.component.scss']
})
export class UserSendOtpComponent {

  public otp: string ='';
  public message : string = ''
  public otpForm: FormGroup;
  public otpSent! : Otp

  constructor(private fb: FormBuilder, protected router: Router, protected serviceAuthentication: AuthenticationService) {
    this.otpForm = this.fb.group({
      otp: [''],
    });
  }

  sendOtp() {
    this.message = '';

    this.otp = this.otpForm.value.otp as string

    this.otp = this.otp.trim();

    this.otpSent= { code : this.otp}

    this.serviceAuthentication.user_send_otp(this.otpSent).subscribe({
        next: (data: InfoLogin)=> {
  
          this.serviceAuthentication.set_info_login(data);

          if (data.change == false){

            this.router.navigate(['/user/profil']);
          }
          else{

            this.router.navigate(['/user/menu']);
          }
          
  
        },
  
        error: err => {
  
          if(err.status <500){
            this.message = err.error.message;
          }
  
          else{
            this.message = 'Erreur interne';
          }
        }
      })

    }
}

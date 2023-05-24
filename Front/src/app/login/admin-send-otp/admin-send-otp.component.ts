import { Component } from '@angular/core';
import { Otp } from '../otp';
import { FormBuilder, FormGroup } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthenticationService } from '../authentication.service';
import { InfoLogin } from '../info-login';

@Component({
  selector: 'app-admin-send-otp',
  templateUrl: './admin-send-otp.component.html',
  styleUrls: ['./admin-send-otp.component.scss']
})
export class AdminSendOtpComponent {

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

    this.serviceAuthentication.admin_send_otp(this.otpSent).subscribe({
      next: (data: InfoLogin)=> {
  
        this.serviceAuthentication.set_info_login(data);

        if (data.change == false){

          this.router.navigate(['/admin/profil']);
        }
        else{

          this.router.navigate(['/admin/menu']);
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

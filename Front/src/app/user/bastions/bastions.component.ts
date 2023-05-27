import { Component } from '@angular/core';
import { UserService } from '../user.service';
import { FormGroup, FormControl } from '@angular/forms';
import { BastionInfo } from 'src/app/admin/bastion-info';
import { NewAdmin } from 'src/app/admin/new-admin';
import { AuthenticationService } from 'src/app/login/authentication.service';

@Component({
  selector: 'app-bastions',
  templateUrl: './bastions.component.html',
  styleUrls: ['./bastions.component.scss']
})
export class BastionsComponent {

  public message: string ='';


  public listBastions : Array<BastionInfo> = new Array<BastionInfo>();


  constructor(protected userService : UserService, protected serviceAuthentication: AuthenticationService) { 

  }

  ngOnInit(): void {


    this.getListBastion()
  }


  getListBastion(){

    this.userService.get_bastions().subscribe({

      next: (data : BastionInfo[]) => {
        
        this.listBastions = data

        
      },
      error: (e) => {
        
        console.error(e)
      },
    })

  }
}

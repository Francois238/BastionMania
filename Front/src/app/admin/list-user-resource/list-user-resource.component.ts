import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';
import { AuthenticationService } from 'src/app/login/authentication.service';
import { AdminInfo } from '../admin-info';
import { AdminService } from '../admin.service';
import { NewAdmin } from '../new-admin';
import { ActivatedRoute, ParamMap } from '@angular/router';
import { UserBastionInfo } from '../user-bastion-info';
import { RessourceInfo } from '../ressource-info';
import { UserInfo } from '../user-info';
import { NewUserBastion } from '../new-user-bastion';
import { ListUserBastionReceived } from '../list-user-bastion-received';

@Component({
  selector: 'app-list-user-resource',
  templateUrl: './list-user-resource.component.html',
  styleUrls: ['./list-user-resource.component.scss']
})
export class ListUserResourceComponent implements OnInit {

  public message: string ='';

  public ajoutForm: FormGroup;
  public mailCrtl: FormControl;

  public userBastion!: NewUserBastion;
  public ressource!: RessourceInfo;

  public bastion_id : string = '';

  public ressource_id : string = '';

  public user!: UserInfo;

  public listUsersBastion : Array<UserBastionInfo> = new Array<UserBastionInfo>();

  constructor(protected adminService : AdminService, protected serviceAuthentication: AuthenticationService,private activRoute: ActivatedRoute) { 

    this.mailCrtl = new FormControl('')
    this.ajoutForm = new FormGroup({
        mail: this.mailCrtl,

    })
  }

  ngOnInit(): void {

    this.mailCrtl = new FormControl('')
    this.ajoutForm = new FormGroup({
        mail: this.mailCrtl,

    })

    this.activRoute.paramMap.subscribe((params: ParamMap) => {
      this.bastion_id = params.get('idBastion') || '';

      this.activRoute.paramMap.subscribe((params: ParamMap) => {
        this.ressource_id = params.get('idRessource') || '';

        this.adminService.get_a_ressource(this.bastion_id, this.ressource_id).subscribe({
          next: (data : any) => {
            this.ressource = data.data as RessourceInfo;
            this.getListUser()
          }
        });

        

      });
    });

  }

  ajoutUser(){
    this.message = '';

    let mail = this.mailCrtl.value.trim() as string;

    this.adminService.get_user_mail(mail).subscribe({
      next: (data : UserInfo[]) => {

        if( data.length == 1){
          this.user= data[0];

          this.userBastion = {
            id: this.user.id,
            ressource_id: this.ressource_id,

          }

          this.adminService.create_user_on_ressource(this.bastion_id, this.ressource_id, this.userBastion).subscribe({

            next: (data : any) => {
                
                this.message="L'utilisateur a bien été ajouté"
  
                this.getListUser()
              },
              error: (e) => {
                this.message="L'utilisateur n'a pas pu être ajouté"
              }
            })
          }

          else{
            this.message="L'utilisateur n'existe pas"
          }
      }

    });


  }

  getListUser(){

    this.adminService.get_users_on_ressource(this.bastion_id, this.ressource_id).subscribe({

      next: (data : ListUserBastionReceived) => {
        
        this.listUsersBastion = data.data as UserBastionInfo[];

        console.log(this.listUsersBastion[0].user_id)

        
      },
      error: (e) => {
        
        console.error(e)
      },
    })

  }

  refreshList(data : string){

    this.getListUser()
  }



}


